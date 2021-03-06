class Query {
  constructor(t,a,b,up) {
    this.type = t
    this.a = a
    this.b = b
    this.up = up
    this.unique = {}
  }

  parent() {
    return this.up ? this.up.parent() : this
  }

  toString() {
    switch (this.type) {
    case 'SET':
    case 'CARD':
    case 'TYPE':
    case 'NAME':
    case 'ORACLE':
    case 'FLAVOR':
    case 'ACTIVATE':
    case 'DISCARD':
    case 'EXILE':
    case 'SACRIFICE':
    case 'TAP':
    case 'UNTAP':
    case 'ARTIST':
    case 'RARITY':
    case 'LAYOUT':
    case 'BORDER':
    case 'LEGAL':
    case 'PT':
    case 'IN':
    case 'FRAME':
      return '('+this.type+' '+this.a.toString()+')'

    case 'DATE':
    case 'UNIQUE':
    case 'FULLART':
    case 'OVERSIZED':
    case 'VARIANT':
    case 'SPOTLIGHT':
    case 'RESERVED':
    case 'REPRINT':
    case 'COLOR':
      return '('+this.type+' '+this.a.string+')'

    case 'EQUIP':
    case 'OWN':
    case 'USD':
    case 'CMC':
    case 'P':
    case 'T':
    case 'CPT':
    case 'PTR':
      return '('+this.type+' '+this.a.string+')'

    case 'NOT':
        return '(NOT '+this.a.toString()+')'

    case 'AND':
    case 'OR':
        return '('+this.type+' '+this.a.toString()+' '+this.b.toString()+')'
    }
  }

  match(card) {
    let _ = {}
    switch (this.type) {
    case 'UNIQUE':
      return this.a.call(card, this.parent().unique)
    case 'SET':
      return this.a == card.set.code
    case 'CARD':
      return this.a == card.id
          || this.a == card.oid
          || this.a == card.number
    case 'TYPE':
      card.type.replace(/—/g, '-'); /* FIXME: normalize lookalike chars */
      return !!this.a.exec(card.type)
    case 'NAME':
      return !!this.a.exec(card.name)
    case 'ORACLE':
      return !!this.a.exec(card.oracle)
    case 'FLAVOR':
      return !!this.a.exec(card.flavor)
    case 'ARTIST':
      return !!this.a.exec(card.artist)
    case 'RARITY':
      return card.flags.indexOf(this.a) >= 0
    case 'DATE':
      return this.a.call(card, card.set.release)
    case 'COLOR':
      return this.a.call(card, card.color)
    case 'P':
        for (let i = 0; i < card.power.length; i++) {
          if (this.a.call(card, card.power[i])) {
            return true
          }
        }
        return false

    case 'T':
        for (let i = 0; i < card.toughness.length; i++) {
          if (this.a.call(card, card.toughness[i])) {
            return true
          }
        }
        return false

    case 'CPT':
        for (let i = 0; i < Math.min(card.power.length, card.toughness.length); i++) {
          if (this.a.call(card, parseInt(card.power[i]) + parseInt(card.toughness[i]))) {
            return true
          }
        }
        return false

    case 'PTR':
        for (let i = 0; i < Math.min(card.power.length, card.toughness.length); i++) {
          if (this.a.call(card, parseInt(card.power[i]) * 1.0 /  parseInt(card.toughness[i]))) {
            return true
          }
        }
        return false

    case 'PT':
        for (let i = 0; i < Math.min(card.power.length, card.toughness.length); i++) {
          if (this.a == card.power[i] + '/' + card.toughness[i]) {
            return true
          }
        }
        return false

    case 'IN':
        return this.a.call(card, card.decks)
    case 'LAYOUT':
        return this.a == card.layout
    case 'BORDER':
        return this.a == card.border
    case 'LEGAL':
        return card.flags.indexOf(this.a) >= 0
    case 'FRAME':
        if (this.a.length > 0 && this.a[0] == '!') { //interior negation
          return !this.a.substr(1).split('').find(l => card.frame.indexOf(l) >= 0)
        } else {
          return !!this.a.split('').find(l => card.frame.indexOf(l) >= 0)
        }
    case 'FULLART':
        return this.a.call(card, card.flags.indexOf('^') >= 0)
    case 'OVERSIZED':
        return this.a.call(card, card.flags.indexOf('O') >= 0)
    case 'VARIANT':
        return this.a.call(card, card.flags.indexOf('~') >= 0)
    case 'SPOTLIGHT':
        return this.a.call(card, card.flags.indexOf('@') >= 0)
    case 'RESERVED':
        return this.a.call(card, card.flags.indexOf('!') >= 0)
    case 'REPRINT':
        return this.a.call(card, card.flags.indexOf('+') >= 0)
    case 'OWN':
      return this.a.call(card, card.owned)
    case 'USD':
      return this.a.call(card, card.price)
    case 'CMC':
      return this.a.call(card, card.cmc)

    case 'EQUIP':
      _.m = card.oracle.match(/\bequip {(\d+)}/i)
      if (!_.m) { return false; }
      return this.a.call(card, parseInt(_.m[1]))

    case 'ACTIVATE':
      _.l = card.oracle.split(/\n+/)
      for (let i = 0; i < _.l.length; i++) {
        _.l[i] = _.l[i].replace(/ *\(.*$/, '')
        _.m = _.l[i].match(/: (.*)/)
        if (_.m && this.a.exec(_.m[1])) { return true; }
      }
      return false

    case 'DISCARD':
      _.l = card.oracle.split(/\n+/)
      for (let i = 0; i < _.l.length; i++) {
        _.l[i] = _.l[i].replace(/ *\(.*$/, '')
        _.m = _.l[i].match(/[^:]*\bdiscard\b.*?: (.*)/i)
        if (_.m && this.a.exec(_.m[1])) { return true; }
      }
      return false

    case 'EXILE':
      _.l = card.oracle.split(/\n+/)
      for (let i = 0; i < _.l.length; i++) {
        _.l[i] = _.l[i].replace(/ *\(.*$/, '')
        _.m = _.l[i].match(/[^:]*\bexile\b.*?: (.*)/i)
        if (_.m && this.a.exec(_.m[1])) { return true; }
      }
      return false

    case 'SACRIFICE':
      _.l = card.oracle.split(/\n+/)
      for (let i = 0; i < _.l.length; i++) {
        _.l[i] = _.l[i].replace(/ *\(.*$/, '')
        _.m = _.l[i].match(/[^:]*\bsacrifice\b.*?: (.*)/i)
        if (_.m && this.a.exec(_.m[1])) { return true; }
      }
      return false

    case 'TAP':
      _.l = card.oracle.split(/\n+/)
      for (let i = 0; i < _.l.length; i++) {
        _.l[i] = _.l[i].replace(/ *\(.*$/, '')
        _.m = _.l[i].match(/[^:]*{T}.*?: (.*)/)
        if (_.m && this.a.exec(_.m[1])) { return true; }
      }
      return false

    case 'UNTAP':
      _.l = card.oracle.split(/\n+/)
      for (let i = 0; i < _.l.length; i++) {
        _.l[i] = _.l[i].replace(/ *\(.*$/, '')
        _.m = _.l[i].match(/[^:]*{Q}.*?: (.*)/)
        if (_.m && this.a.exec(_.m[1])) { return true; }
      }
      return false

    case 'NOT':
      return !this.a.match(card)
    case 'AND':
      return this.a.match(card) && this.b.match(card)
    case 'OR':
      return this.a.match(card) || this.b.match(card)
    }
  }
}

function tokenize(s) {
  let autobool = function (predicate) {
    return [['QUALIFIER', predicate],
            ['IDENTIFIER', 'y']]
  }
  let keywords = {
    AND:   [['AND']],
    OR:    [['OR']],
    NOT:   [['NOT']],

    OWNED: [['QUALIFIER','OWN'],
            ['IDENTIFIER', '1+']],
    HAVE:  [['QUALIFIER','OWN'],
            ['IDENTIFIER', '1+']],
    NEED:  [['QUALIFIER','OWN'],
            ['IDENTIFIER', '0']],

    UNIQUE: [['QUALIFIER', 'UNIQUE'],
             ['IDENTIFIER', 'card']],

    REPRINT:   autobool('REPRINT'),
    RESERVED:  autobool('RESERVED'),
    SPOTLIGHT: autobool('SPOTLIGHT'),
    FULLART:   autobool('FULLART'),
    OVERSIZED: autobool('OVERSIZED'),
    VARIANT:   autobool('VARIANT'),
  }

  let aliases = {
    POWER:  'P',
    ATTACK: 'P',

    TOUGHNESS: 'T',
    DEFENSE:   'T'
  }

  let qualify = function (q) {
    q = q.toUpperCase()
    return aliases[q] || q
  }

  let kw = []
  for (let k in keywords) { kw.push(k); }
  let kre = new RegExp("^("+kw.join('|')+")\\b(?!:)", 'i')

  let tok = []
parsing:
  while (s.length > 0) {
    if (s.match(/^\s+/)) {
      s = s.replace(/^\s+/, '')
      continue parsing
    }

    let m = []
    switch (s[0]) {
    case '"':
    case '\'':
      /* quoted string */
      for (let i = 1; i < s.length; i++) {
        if (s[i] == s[0]) {
          /* FIXME: no escape quoting yet */
          tok.push(['STRING', s.substr(1, i-1)])
          s = s.substr(i+1)
          continue parsing
        }
      }
      throw 'unterminated quoted string'

    case '(':
    case ')':
        tok.push([s[0]])
        s = s.substr(1)
        continue parsing

    case '@':
      tok.push(['QUALIFIER', 'COLOR'])
      s = s.substr(1)
      continue parsing

    case '+':
      tok.push(['QUALIFIER', 'ORACLE'])
      s = s.substr(1)
      continue parsing

    case '=':
      tok.push(['QUALIFIER', 'RARITY'])
      s = s.substr(1)
      continue parsing

    case '!':
      tok.push(['NOT'])
      s = s.substr(1)
      continue parsing
    }

    m = s.match(kre)
    if (m) {
      let toks = keywords[m[1].toUpperCase()]
      for (let i = 0; i < toks.length; i++) {
        tok.push(toks[i])
      }
      s = s.replace(kre, '')
      continue parsing
    }

    let re = new RegExp('^([a-zA-Z0-9_-]+):\\s*')
    m = s.match(re)
    if (m) {
      s = s.replace(re, '')
      tok.push(['QUALIFIER', qualify(m[1])])
      continue parsing
    }

    re = new RegExp('^([^\\s()]+)\\s*')
    m = s.match(re)
    if (m) {
      s = s.replace(re, '')
      tok.push(['IDENTIFIER', m[1]])
      continue parsing
    }

    throw 'unrecognized query fragment: ['+s.substr(0,50)+'...]'
  }

  /* collapse adjacent IDENTIFIER tokens */
  let collapsed = [],
      last = tok[0];
  for (let i = 1; i < tok.length; i++) {
    if (last[0] == 'IDENTIFIER' && tok[i][0] == last[0]) {
      last[1] += ' ' + tok[i][1]
    } else {
      collapsed.push(last)
      last = tok[i]
    }
  }
  collapsed.push(last)
  return collapsed
}

function parse(tok) {
  let strict_re = function (v) { return new RegExp('\\b'+v+'\\b'); },
      loose_re  = function (v) { return new RegExp('\\b'+v+'\\b', 'i'); },
      setcode   = function (v) { return v.toUpperCase(); },
      literal   = function (v) { return v.toLowerCase(); },
      rarity    = function (v) {
        switch (v.toLowerCase()) {
        case 'c': case 'common':     return '1'
        case 'u': case 'uncommon':   return '2'
        case 'r': case 'rare':       return '3'
        case 'm': case 'mythic':     return '4'
        default: return ' '
        }
      },
      legalese  = function (v) {
        switch (v.toLowerCase()) {
        case 'brawl':      return 'B'
        case 'edh':
        case 'commander':  return 'E'
        case 'duel':       return 'd'
        case 'frontier':   return 'j'
        case 'future':     return 'f'
        case 'historic':   return 'h'
        case 'legacy':     return 'l'
        case 'modern':     return 'm'
        case 'old-school':
        case 'old_school': return 'o'
        case 'pauper':     return 'P'
        case 'penny':      return '$'
        case 'pioneer':    return 'p'
        case 'standard':   return 's'
        case 'vintage':    return 'v'
        default:           return ' '
        }
      },
      framer = function (v) {
        let m, op, point
        switch (v.toLowerCase()) {
        case '1993':
        case '93':            return '3'
        case '1997':
        case '97':            return '7'
        case 'modern':
        case '2003':
        case '03':            return 'M'
        case '2015':
        case '15':
        case 'current':       return 'N'
        case 'future':
        case 'timeshifted':
        case 'time-shifted':  return 'F'

        case 'legendary':     return 'L'
        case 'miracle':       return 'm'
        case 'nyx':
        case 'nyxtouched':
        case 'nyx-touched':   return 'n'
        case 'draft':         return 'D'
        case 'devoid':        return 'd'
        case 'tombstone':
        case 'flashback':     return 't'
        case 'colorshifted':
        case 'color-shifted': return '$'
        case 'showcase':      return 's'
        case 'compass':       return 'c'
        case 'extended':
        case 'extendedart':
        case 'extended-art':
        case 'stretched':     return '+'
        case 'companion':     return 'C'

        case 'shifted':       return '$F'
        case 'special':       return  'Dts+'
        case 'normal':        return '!Dts+'
        case 'old':           return '37'
        case 'new':           return 'MNF'

        default:
          op = ''
          point = ''
          m = v.match(new RegExp('^([<>]=?)([a-zA-Z0-9]+)$'))
          if (m) {
            op = m[1]
            point = m[2]
          }
          m = v.match(new RegExp('^([a-zA-Z0-9]+)\\+$'))
          if (m) {
            op = '>='
            point = m[1]
          }

          switch (point) {
          case '1993':
          case '93':
            switch (op) {
            case '>':  return '7MNF'
            case '>=': return '37MNF'
            case '<':  return ''
            case '<=': return '3'
            }
            break

          case '1997':
          case '97':
            switch (op) {
            case '>':  return 'MNF'
            case '>=': return '7MNF'
            case '<':  return '3'
            case '<=': return '37'
            }
            break

          case '2003':
          case '03':
          case 'modern':
            switch (op) {
            case '>':  return 'NF'
            case '>=': return 'MNF'
            case '<':  return '37'
            case '<=': return '37M'
            }
            break

          case '2015':
          case '15':
          case 'current':
            switch (op) {
            case '>':  return 'F'
            case '>=': return 'NF'
            case '<':  return '37M'
            case '<=': return '37MN'
            }
            break

          case 'future':
            switch (op) {
            case '>':  return ''
            case '>=': return 'F'
            case '<':  return '37MN'
            case '<=': return '37MNF'
            }
            break
          }
        }
        return ''
      },
      boolish   = function (v) {
        let fn = function (v) { return !v; }
        fn.string = 'no'
        switch (v.toLowerCase()) {
        case 'y':
        case 'yes':
        case '1':   fn = function (v) { return !!v; }
                    fn.string = 'yes'
        }
        return fn
      },
      colorish  = function (v) {
        v = v.toUpperCase()

        let ck = function (w,u,b,r,g) {
          let fn = function (c) {
            let map = {}
            for (let i = 0; i < c.length; i++) { map[c[i]] = true; }
            return !!map.W == !!w &&
                   !!map.U == !!u &&
                   !!map.B == !!b &&
                   !!map.R == !!r &&
                   !!map.G == !!g
          }
          fn.string = v
          return fn
        }

        switch (v) {
        case "NONE":     /* '' */  return ck(0, 0, 0, 0, 0)

        case "WHITE":    /* W */   return ck(1, 0, 0, 0, 0)
        case "BLUE":     /* U */   return ck(0, 1, 0, 0, 0)
        case "BLACK":    /* B */   return ck(0, 0, 1, 0, 0)
        case "RED":      /* R */   return ck(0, 0, 0, 1, 0)
        case "GREEN":    /* G */   return ck(0, 0, 0, 0, 1)

        case "AZORIUS":  /* WU */  return ck(1, 1, 0, 0, 0)
        case "DIMIR":    /* UB */  return ck(0, 1, 1, 0, 0)
        case "RAKDOS":   /* BR */  return ck(0, 0, 1, 1, 0)
        case "GRUUL":    /* RG */  return ck(0, 0, 0, 1, 1)
        case "SELESNYA": /* WG */  return ck(1, 0, 0, 0, 1)
        case "ORZHOV":   /* WB */  return ck(1, 0, 1, 0, 0)
        case "IZZET":    /* UR */  return ck(0, 1, 0, 1, 0)
        case "GOLGARI":  /* BG */  return ck(0, 0, 1, 0, 1)
        case "BOROS":    /* WR */  return ck(1, 0, 0, 1, 0)
        case "SIMIC":    /* UG */  return ck(0, 1, 0, 0, 1)

        case "BANT":     /* WUG */ return ck(1, 1, 0, 0, 1)
        case "ESPER":    /* WUB */ return ck(1, 1, 1, 0, 0)
        case "GRIXIS":   /* UBR */ return ck(0, 1, 1, 1, 0)
        case "JUND":     /* BRG */ return ck(0, 0, 1, 1, 1)
        case "NAYA":     /* WRG */ return ck(1, 0, 0, 1, 1)

        case "ABZAN":    /* WBG */ return ck(1, 0, 1, 0, 1)
        case "JESKAI":   /* WUR */ return ck(1, 1, 0, 1, 0)
        case "SULTAI":   /* UBG */ return ck(0, 1, 1, 0, 1)
        case "MARDU":    /* WBR */ return ck(1, 0, 1, 1, 0)
        case "TEMUR":    /* URG */ return ck(0, 1, 0, 1, 1)
        }

        let fn = function (c) {
          for (let i = 0; i < v.length; i++) {
            if (c.indexOf(v[i]) < 0) { return false; }
          }
          return true
        }
        fn.string = v
        return fn
      },
      deckish   = function (v) {
        v = v.toLowerCase()
        let fn = function (decks) {
          return decks && decks[v]
        }
        fn.string = v
        return fn
      },
      range     = function (v) {
        if (typeof(v) === 'undefined') {
          return false
        }
        let n, op, fn = function () { return false; }
        let m = v.match('^([<>]?=?)?([0-9]+(\\.[0-9]+)?)$')
        if (m) {
          op = m[1] || '='
          n = parseFloat(m[2])
        } else {
          m = v.match('^([0-9]+(\\.[0-9]+)?)\\+$')
          if (m) {
            op = '>='
            n = parseFloat(m[1])
          }
        }

        switch (op) {
        case '>':  fn = function (v) { return v >  n; }; break
        case '<':  fn = function (v) { return v <  n; }; break
        case '>=': fn = function (v) { return v >= n; }; break
        case '<=': fn = function (v) { return v <= n; }; break
        case '=':  fn = function (v) { return v == n; }; break
        }
        fn.string = v
        return fn
      },
      daterange = function (v) {
        if (typeof(v) === 'undefined') {
          return false
        }
        let a, b, ts, op, fn = function () { return false; }
        let m = v.match('^([<>]?=?)?([0-9-]+)$')
        if (m) {
          op = m[1] || '='
          ts = m[2]

        } else {
          m = v.match('^([0-9-]+)\\+$')
          if (m) {
            op = '>='
            ts = m[1]

          } else {
            return false
          }
        }

        if (ts.length == 4) {
          a = ts + "0101"
          b = ts + "1231"

        } else if (ts.length == 6) {
          a = ts + "01"
          b = ts + "31"; // close enough

        } else if (ts.length == 8) {
          a = ts
          b = ts

        } else {
          return false
        }

        a = parseInt(a)
        b = parseInt(b)

        switch (op) {
        case '>':  fn = function (v) { return v >  b; }; break
        case '<':  fn = function (v) { return v <  a; }; break
        case '>=': fn = function (v) { return v >= a; }; break
        case '<=': fn = function (v) { return v <= b; }; break
        case '=':  fn = function (v) { return v >= a && v <= b; }; break
        }
        fn.string = v
        return fn
      },
      uniquify  = function (v) {
        v = v.toLowerCase()

        //let uniques = {card: {}, art: {}}
        let fn = function (unique) {
          let k
          switch (v) {
          case 'card': k = this.oid; break
          case 'art':  k = this.art; break
          }
          let rc = !(k in unique)
          unique[k] = 1
          return rc
        }
        fn.string = v
        return fn
      },
      data      = [],
      ops       = [],
      prec      = {
        'AND': 1,
        'OR':  1,
        'NOT': 2
      }

  while (tok.length > 0) {
    let t = tok.shift()
    let v, fn, z, a, b
    switch (t[0]) {
    case 'IDENTIFIER':
      data.push(new Query('NAME', new RegExp('\\b'+t[1]+'\\b', 'i')))
      break

    case 'STRING':
      data.push(new Query('NAME', new RegExp('\\b'+t[1]+'\\b')))
      break

    case 'QUALIFIER':
      v = tok.shift()

      /* handle predictes that don't _need_ a value... */
      switch (t[1]) {
        case 'ACTIVATE':
        case 'DISCARD':
        case 'EXILE':
        case 'SACRIFICE':
        case 'TAP':
        case 'UNTAP':
          if (v) {
            switch (v[0]) {
            case 'IDENTIFIER':
            case 'STRING': break
            default:
              tok.unshift(v)
              v = undefined
              break
            }
          }
          if (!v) {
            v = ['IDENTIFIER', '']
          }
      }

      switch (v[0]) {
      case 'IDENTIFIER': fn = loose_re;  break
      case 'STRING':     fn = strict_re; break
      default:
        throw 'bad value for '+t[1]+' qualifier'
      }

      switch (t[1]) {
      case 'UNIQUE':  fn = uniquify; break
      case 'SET':     fn = setcode;  break
      case 'CARD':
      case 'LAYOUT':
      case 'BORDER':
      case 'PT':      fn = literal;  break
      case 'RARITY':  fn = rarity;   break
      case 'LEGAL':   fn = legalese; break
      case 'FRAME':   fn = framer;   break
      case 'FULLART':
      case 'OVERSIZED':
      case 'VARIANT':
      case 'SPOTLIGHT':
      case 'RESERVED':
      case 'REPRINT': fn = boolish;  break
      case 'COLOR':   fn = colorish; break
      case 'IN':      fn = deckish;  break
      case 'EQUIP':
      case 'OWN':
      case 'USD':
      case 'P':
      case 'T':
      case 'CPT':
      case 'PTR':
      case 'CMC':     fn = range;    break
      case 'DATE':    fn = daterange; break
      }
      data.push(new Query(t[1], fn(v[1])))
      break

    case 'AND':
    case 'OR':
    case 'NOT':
      z = ops.length - 1
      while (z >= 0 && ops[z] != '(' && prec[ops[z]] >= prec[t[0]]) {
        let op = ops.pop(); z--
        switch (op) {
        case 'NOT':
          if (data.length < 1) { throw 'stack underflow (data) for '+op+' op'; }
          data.push(new Query(op, data.pop()))
          break

        case 'OR':
        case 'AND':
          if (data.length < 2) { throw 'stack underflow (data) for '+op+' op'; }
            b = data.pop()
            a = data.pop()
            data.push(new Query(op, a, b))
            break
        }
      }
      ops.push(t[0])
      break

    case '(':
      ops.push(t[0])
      break

    case ')':
      z = ops.length - 1
      while (z >= 0 && ops[z] != '(') {
        /* ---->8--------------- */
        let op = ops.pop(); z--
        switch (op) {
        case 'NOT':
          if (data.length < 1) { throw 'stack underflow (data) for '+op+' op'; }
          data.push(new Query(op, data.pop()))
          break

        case 'OR':
        case 'AND':
          if (data.length < 2) { throw 'stack underflow (data) for '+op+' op'; }
            b = data.pop()
            a = data.pop()
            data.push(new Query(op, a, b))
            break
        }
        /* ---->8--------------- */
      }
      if (z < 0) { throw 'mismatched parentheses'; }
      ops.pop()
      break

    default:
      console.log('no handler for a '+t[0]+' yet...')
    }
  }

  while (ops.length > 0) {
    let op = ops.pop()
    let a, b
    switch (op) {
    case 'NOT':
      if (data.length < 1) { throw 'stack underflow (data) for '+op+' op'; }
      data.push(new Query(op, data.pop()))
      break

    case 'OR':
    case 'AND':
      if (data.length < 2) { throw 'stack underflow (data) for '+op+' op'; }
      b = data.pop()
      a = data.pop()
      data.push(new Query(op, a, b))
      break

    case '(':
    case ')':
      throw 'mismatched parentheses'
    }
  }
  if (data.length != 1) {
    throw 'syntax error'
  }
  return data[0]
}

Query.parse = function (s) {
  return parse(tokenize(s))
}

export default Query
