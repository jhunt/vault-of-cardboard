import API from './api'
import Draft from './draft'
import CDIF from './cdif'
import Query from './query'
import When from './when'
import Fuse from 'fuse.js'

const CollectionLoaded = 'vault:collection-loaded'
const CardsLoaded      = 'vault:cards-loaded'

function flagged(v, map, fallback) {
  for (var k in map) {
    if (v.indexOf(k) >= 0) {
      return map[k]
    }
  }
  return fallback
}

class Vault {
  constructor() {
    this.index = {}
    this.cards = {}
    this.sets  = []

    this.promise = Promise.resolve()
  }

  on(events, fn) {
    When(events, fn)
  }

  ingest(data, prices) {
    this.index = {}
    this.cards = {}
    this.sets  = []

    let search = []; // for storing fuse indexing subset documents.

    for (var code in data.sets) {
      this.sets.push({
        name:    data.sets[code].name,
        size:    data.sets[code].cards.length,
        code:    data.sets[code].code,
        release: data.sets[code].released_at.replace(/-/g, '')
      })

      this.cards[code] = []
      let named = {}
      for (var i = 0; i < data.sets[code].cards.length; i++) {
        var card = data.sets[code].cards[i]
        card = {
          id        : card.id,
          oid       : card.oid,
          flags     : card.flags + data.cards[card.oid].legal, // this is new
          artist    : card.artist,
          layout    : card.layout,
          frame     : card.frame,
          number    : card.number,
          border    : card.border == 'borderless' ? 'none' : card.border,

          // for draft
          rarity    : flagged(card.flags, {
                          '1': 'common',
                          '2': 'uncommon',
                          '3': 'rare',
                          '4': 'mythic'
                        }, 'unknown rarity'),

          image     : data.sets[code].code + "/" + data.sets[code].code + "-" + card.id + ".jpg",

          name      : data.cards[card.oid].name,
          type      : data.cards[card.oid].type_line,
          oracle    : data.cards[card.oid].text,

          cmc       : data.cards[card.oid].cmc,
          cost      : data.cards[card.oid].mana_cost,
          color     : data.cards[card.oid].color_identity.join(''),

          price     : (prices && (card.id in prices)) ? prices[card.id] : undefined,
          owned     : 0,         // we will backpatch this later, with collection.json data

          flavor    : card.flavor,
          power     : (data.cards[card.oid].power || '').split("//"),
          toughness : (data.cards[card.oid].tough || '').split("//"),
          art       : card.illustration,

          back      : card.layout == 'transform' || card.layout == 'modal_dfc' || card.layout == 'double_faced_token'
                    ? data.sets[code].code + "/" + data.sets[code].code + "-" + card.id + ".flip.jpg"
                    : "",

          set: {
            name    : data.sets[code].name,
            code    : data.sets[code].code,
            total   : data.sets[code].cards.length,
            release : data.sets[code].released_at.replace(/-/g, '')
          },
        }
        // track set name collisions
        if (!(card.name in named)) { named[card.name] = []; }
        named[card.name].push(card.id)

        // keep track of the card itself
        this.cards[code].push(card)

        // inform the search engine
        search.push({
          needle: card.name.toLowerCase(),
          set:    code,
          number: card.number,
          name:   card.name,
          id:     card.id
        })
      }
      for (let i = 0; i < this.cards[code].length; i++) {
        // count up set-wide collisions
        this.cards[code][i].others = named[this.cards[code][i].name].length - 1

        // track the card in the index, for quick lookups
        this.index[this.cards[code][i].id] = this.cards[code][i]
      }
    }

    this.fuse = new Fuse(search, {
      keys: ['needle'],
      includeScore: true,
      threshold: 0.4
    })

    When.trigger(CardsLoaded)
    return this
  }

  draft(set) {
    let lands = set
    while (lands && !this.has_any('set:'+lands+' and type:basic land')) {
      lands = this.previous_set(lands)
    }
    console.log('drafting %s, with lands from %s...', set, lands)
    return new Draft(this, {set, lands})
  }

  update_prices(prices) {
    for (let id in prices) {
      this.index[id].price = prices[id]
    }
    return this
  }

  has(id) {
    return typeof(this.index[id]) !== 'undefined'
  }

  card(id) {
    return this.index[id]
  }

  previous_set(set) {
    let sets = this.sets.sort((a,b) => parseInt(a.release) - parseInt(b.release))
    for (let i = 1; i < sets.length; i++) {
      if (sets[i].code == set) {
        return sets[i-1].code
      }
    }
    return undefined
  }

  find(set, name) {
    if (!this.cards[set]) {
      return undefined
    }
    for (let i = 0; i < this.cards[set].length; i++) {
      if (this.cards[set][i].name == name) {
        return this.cards[set][i]
      }
    }
    return undefined
  }

  clear_collection() {
    for (var id in this.index) {
      this.index[id].owned = 0
    }
  }

  unload_collection() {
    this.clear_collection()
    When.clear(CollectionLoaded)
  }

  no_collection() {
    this.load_collection([], [])
  }

  load_collection(base, patches) {
    this.clear_collection()
    base.forEach((card) => {
      if (card[1].pid in this.index) {
        this.index[card[1].pid].owned = card[0]
      }
    })

    patches.forEach(patch => {
      patch.forEach(card => {
        if (card.id in this.index) {
          this.index[card.id].owned += card.quantity
        } else {
          console.log('unable to update own:%d for card [%s] - card not found in vault', card.quantity, card.id)
        }
      })
    })

    When.trigger(CollectionLoaded)
    return this
  }

  has_any(q) {
    var query = Query.parse(q)
    for (var set in this.cards) {
      for (var i = 0; i < this.cards[set].length; i++) {
        if (query.match(this.cards[set][i])) {
          return true
        }
      }
    }
    return false
  }

  filter(cards, q) {
    var query = Query.parse(q)
    var found = []
    cards.forEach(card => {
      if (query.match(card)) {
        found.push(card)
      }
    })
    return found
  }

  search(q, limit) {
    var query = Query.parse(q)
    var found = []
    for (var set in this.cards) {
      for (var i = 0; i < this.cards[set].length; i++) {
        if (query.match(this.cards[set][i])) {
          found.push(this.cards[set][i])
          limit--
          if (limit == 0) { break; }
        }
      }
      if (limit == 0) { break; }
    }

    found = found.sort(function (a, b) {
      if (a.name > b.name) { return 1; }
      if (a.name < b.name) { return -1; }
      return 0
    })

    return found
  }

  resolve(cdif) {
    let n = 1
    let pile = []
    CDIF.parse(cdif).forEach(line => {
      let card = this.find(line.set, line.oracle)
      if (!card) {
        throw new Error('card ['+line.set+'] "'+line.oracle+'" not found in vault.')
      }
      Array.from({ length: line.quantity }).forEach(() => pile.push({ id: n++, card: card }))
    })
    return pile
  }

  clarify(set, number, name) {
    let res = []
    if (set in this.cards) {
      for (let i = 0; i < this.cards[set].length; i++) {
        let card = this.cards[set][i]
        if (card.name == name && (!number || number == card.number)) {
          res.push(card)
        }
      }
      if (res.length == 1) {
        return res[0]
      }
    }

    name = name.toLowerCase()
    return this.fuse.search(name).map(r => {
      let local = set && r.item.set == set
      return {
        set    : r.item.set,
        number : r.item.number,
        name   : r.item.name,
        id     : r.item.id,
        score  : parseInt(r.score / 1000 * (local ? 2 : 1)),
        type   : local ? 'in-set' : 'global'
      }
    }).sort((a, b) => { return b.score - a.score; })
  }
}

export default {
  API,
  CDIF,
  Query,
  Vault,
  CollectionLoaded,
  CardsLoaded,
  AllLoaded: [CardsLoaded, CollectionLoaded]
}
