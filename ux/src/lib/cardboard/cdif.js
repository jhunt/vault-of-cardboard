let CDIF = {
  parse(src) {
    let lines = []
    let lineno = 0

    src.split(/\n/).forEach((line) => {
      lineno++

      let state = 0
      let quantity = 0
      let set = ''
      let number = ''
      let name = ''

      for (let i = 0; i < line.length; i++) {
        let c = line.charAt(i)

        if (state == 0 && c == ' ') {
          // |. 1x MIR Island|
          // ignore whitespace
        } else if (state == 0 && c >= '0' && c <= '9') {
          // |.1x MIR Island|
          state = 1
          quantity = c - '0'
        } else if (state == 0 && c == '#') {
          // |.# some comment |
          state = 14
        } else if (state == 1 && c >= '0' && c <= '9') {
          // |2.0x MIR Island|
          quantity = quantity * 10 + (c - '0')
        } else if (state == 1 && c == 'x') {
          // |1.x MIR Island|
          state = 2
        } else if (state == 1 && c == ' ') {
          // |1. MIR Island|
          state = 3
        } else if (state == 2 && c == ' ') {
          // |1x. MIR Island|
          state = 3
        } else if (state == 3 && c == ' ') {
          // |1x . MIR Island|
          // ignore whitespace
        } else if (state == 3 && ((c >= 'a' && c <= 'z')
                               || (c >= 'A' && c <= 'Z')
                               || (c >= '0' && c <= '9'))) {
          // |1x .MIR Island|
          set += c
          state = 4
        } else if (state == 4 && ((c >= 'a' && c <= 'z')
                               || (c >= 'A' && c <= 'Z')
                               || (c >= '0' && c <= '9'))) {
          // |1x M.IR Island|
          set += c
        } else if (state == 4 && c == ' ') {
          // |1x MIR. Island|
          state = 5
        } else if (state == 5 && c == ' ') {
          // |1x MIR . Island|
          // ignore whitespace
        } else if (state == 5 && c == '*') {
          // |1x MIR .*335 Island|
          state = 15
        } else if (state == 5 && ((c >= 'a' && c <= 'z')
                               || (c >= 'A' && c <= 'Z')
                               || (c == '+') // for "+2 Mace" from AFR
                               || (c >= '0' && c <= '9'))) {
          // |1x MIR .Island|
          name += c
          state = 6
        } else if (state == 6 && c == '#') {
          // |1x MIR Island .# the best island|
          state = 14
        } else if (state == 6 && c == '|') {
          // |1x MIR Island .| foil|
          state = 7
        } else if (state == 6) {
          // |1x MIR Isl.and|
          name += c
        } else if (state == 7 && c == ' ') {
          // |1x MIR Island |. foil|
          // ignore whitespace
        } else if (state == 7 && ((c >= 'a' && c <= 'z')
                               || (c >= 'A' && c <= 'Z')
                               || (c >= '0' && c <= '9'))) {
          // |1x MIR Island | .foil|
          //gvar += c
          state = 8
        } else if (state == 7 && c == '(') {
          // |1x MIR Island | .(signed:maro)|
          state = 9
        } else if (state == 7 && c == '#') {
          // |1x MIR Island | .#(ignored:yes)|
          state = 14
        } else if (state == 8 && c == ' ') {
          // |1x MIR Island | foil. misprint|
          //gvars.push(gvar + c)
          //gvar = ''
          state = 7
        } else if (state == 8 && ((c >= 'a' && c <= 'z')
                               || (c >= 'A' && c <= 'Z')
                               || (c >= '0' && c <= '9'))) {
          // |1x MIR Island | foil .misprint|
          //gvar += c
        } else if (state == 9 && ((c >= 'a' && c <= 'z')
                               || (c >= 'A' && c <= 'Z')
                               || (c >= '0' && c <= '9'))) {
          // |1x MIR Island | (.signed:artist)|
          // lkey = c
          state = 10
        } else if (state == 10 && ((c >= 'a' && c <= 'z')
                                || (c >= 'A' && c <= 'Z')
                                || (c >= '0' && c <= '9'))) {
          // |1x MIR Island | (si.gned:artist)|
          // lkey = c
          // lkey += c
        } else if (state == 10 && c == ':') {
          // |1x MIR Island | (signed.:artist)|
          state = 11
        } else if (state == 11 && c == ' ') {
          // |1x MIR Island | (signed:. artist)|
          // ignore whitespace
        } else if (state == 11 && ((c >= 'a' && c <= 'z')
                                || (c >= 'A' && c <= 'Z')
                                || (c >= '0' && c <= '9'))) {
          // |1x MIR Island | (signed:.artist)|
          //lval = c
          state = 12
        } else if (state == 12 && c == ')') {
          // |1x MIR Island | (signed:artist.)|
          //lvars.push([lkey, lval])
          state = 13
        } else if (state == 12) {
          // |1x MIR Island | (signed:a.rtist)|
          //lval += c
        } else if (state == 13 && c == ' ') {
          // |1x MIR Island | (signed:artist) .(promo:1996)|
          state = 7
        } else if (state == 13 && c == '#') {
          // |1x MIR Island | (signed:artist) # awww yeah|
          state = 14
        } else if (state == 14) {
          // |#. this is ignored|
          // ignore comments
        } else if (state == 15 && (c >= '0' && c <= '9')) {
          // |1x MIR *.335 Island|
          number += c
          state = 16
        } else if (state == 16 && ((c >= 'a' && c <= 'z')
                                || (c >= 'A' && c <= 'Z')
                                || (c >= '0' && c <= '9'))) {
          // |1x MIR *3.35 Island|
          number += c
        } else if (state == 16 && c == ' ') {
          // |1x MIR *335. Island|
          state = 17
        } else if (state == 17 && c == ' ') {
          // |1x MIR *335 . Island|
          // ignore whitespace
        } else if (state == 17 && ((c >= 'a' && c <= 'z')
                                || (c >= 'A' && c <= 'Z')
                                || (c >= '0' && c <= '9'))) {
          // |1x MIR *335 .Island|
          name += c
          state = 6
        } else {
          throw new Error('syntax error on line '+lineno.toString()+': unexpected character "'+c+'" (state '+state+')')
        }
      }

      name = name.replace(/\s+$/, '')

      if (state == 8) {
        //gvars.push(gvar)
        state = 14
      }

      if (state == 0 || state == 6 || state == 7 || state == 8 || state == 13 || state == 14) {
        if (quantity) {
          lines.push({
            line:     lineno,
            src:      line,
            quantity: quantity,
            set:      set,
            number:   number,
            oracle:   name
          })
        }
      } else {
        throw new Error('syntax error on line '+lineno.toString()+' '+JSON.stringify({s:state,eof:true}))
      }
    })

    return lines
  },

  validate(cdif, vault, max) {
    let probs = []

    let lines = CDIF.parse(cdif)
    for (let i = 0; i < lines.length; i++) {
      let l = lines[i]
      let card = vault.clarify(l.set, l.number, l.oracle)
      if (card instanceof Array) {

        // figure out the two "halves" of a line - everything BEFORE
        // the pipe delimiter, and everything after.  clarifiers can
        // (and should!) use these two to show diffs / patches.
        let halves = l.src.split(/\s*\|\s*/)
        if (halves[1]) {
          halves[1] = ` | ${halves[1]}`
        }

        probs.push({
          id     : l.line + ': ' + l.src,
          line   : l.line,
          target : l.oracle + (l.set ? ' (in '+l.set+')' : ''),
          src    : l.src,
          value  : halves[0],
          vars   : halves[1],
          error  : 'not found via exact(ish) match.  Did you mean one of these instead?',
          cards  : card.slice(0,8).map(c => vault.card(c.id))
        })

        if (max && probs.length == max) {
          return probs
        }
      }
    }

    return probs
  }
}

export default CDIF
