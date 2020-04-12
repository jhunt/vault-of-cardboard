const Query = require('./query.js').Query,
      When  = require('./when.js').When,
      Fuse  = require('fuse.js');

const CollectionLoaded = 'collection-loaded';
const CardsLoaded      = 'cards-loaded';

function flagged(v, map, fallback) {
  for (var k in map) {
    if (v.indexOf(k) >= 0) {
      return map[k];
    }
  }
  return fallback;
}

let Vault = class {
  constructor() {
    this.index = {};
    this.cards = {};
    this.sets  = [];

    this.promise = Promise.resolve();
  }

  ingest(data) {
    this.index = {};
    this.cards = {};
    this.sets  = [];

    let fuse = []; // for storing fuse indexing subset documents.

    for (var code in data.sets) {
      this.sets.push({
        name:    data.sets[code].name,
        size:    data.sets[code].cards.length,
        code:    data.sets[code].code,
        release: data.sets[code].released_at.replace(/-/g, '')
      });

      this.cards[code] = [];
      for (var i = 0; i < data.sets[code].cards.length; i++) {
        var card = data.sets[code].cards[i];
        card = {
          id        : card.id,
          oid       : card.oid,
          flags     : card.flags + data.cards[card.oid].legal, // this is new
          artist    : card.artist,
          layout    : card.layout,
          // frame unused
          number    : card.number,
          // border unused

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

          price     : undefined, // we will backpatch this later, with prices.json data
          owned     : 0,         // we will backpatch this later, with collection.json data

          flavor    : card.flavor,
          power     : data.cards[card.oid].power,
          toughness : data.cards[card.oid].tough,
          pt        : (data.cards[card.oid].power && data.cards[card.oid].tough
                      ? data.cards[card.oid].power + "/" + data.cards[card.oid].tough
                      : ""),
          art       : card.illustration,

          back      : "", // FIXME - seems to be missing

          set: {
            name  : data.sets[code].name,
            code  : data.sets[code].code,
            total : data.sets[code].cards.length,
          },
        };
        this.cards[code].push(card);
        this.index[card.id] = card;
        fuse.push({
          set:   code,
          name:  card.name,
          id:    card.id
        });
      }
    }

    this.fuse = new Fuse(fuse, {
      keys: ['name'],
      includeScore: true,
      threshold: 0.4
    });

    return this.trigger(CardsLoaded);
  }

  has(id) {
    return typeof(this.index[id]) !== 'undefined';
  }

  card(id) {
    return this.index[id];
  }

  clear_collection() {
    for (var id in this.index) {
      this.index[id].owned = 0;
    }
  }

  unload_collection() {
    this.clear_collection();
    When.clear(CollectionLoaded);
  }

  no_collection() {
    this.load_collection([], []);
  }

  load_collection(base, patches) {
    this.clear_collection();
    base.forEach((card) => {
      if (card[1].pid in this.index) {
        this.index[card[1].pid].owned = card[0];
      }
    });

    patches.forEach(patch => {
      patch.forEach(card => {
        if (card.id in this.index) {
          this.index[card.id].owned += card.quantity;
        }
      });
    });

    return this.trigger(CollectionLoaded);
  }

  search(q, limit) {
    var query = Query.parse(q);
    var found = [];
    for (var set in this.cards) {
      for (var i = 0; i < this.cards[set].length; i++) {
        if (query.match(this.cards[set][i])) {
          found.push(this.cards[set][i]);
          limit--;
          if (limit == 0) { break; }
        }
      }
      if (limit == 0) { break; }
    }

    found = found.sort(function (a, b) {
      if (a.name > b.name) { return 1; }
      if (a.name < b.name) { return -1; }
      return 0;
    });

    return found;
  }

  clarify(set, name) {
    if (set in this.cards) {
      for (let i = 0; i < this.cards[set].length; i++) {
        let card = this.cards[set][i];
        if (card.name == name) {
          return card;
        }
      }
    }

    return this.fuse.search(name).map(r => {
      return {
        set   : r.item.set,
        name  : r.item.name,
        id    : r.item.id,
        score : r.score,
        type  : (set && r.item.set == set) ? 'in-set' : 'global'
      };
    }).sort((a, b) => { return a.score - b.score; });
  }

  when(events, fn) {
    When(events, fn);
    return this;
  }

  trigger(ev) {
    When.trigger(ev);
    return this;
  }
};

module.exports = Object.freeze({
  Vault            : Vault,
  CollectionLoaded : CollectionLoaded,
  CardsLoaded      : CardsLoaded,
  AllLoaded        : [CardsLoaded, CollectionLoaded]
});
