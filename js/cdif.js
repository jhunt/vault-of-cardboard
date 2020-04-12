class CDIF {
}

CDIF.parse = function (src) {
  let lines = [];
  let lineno = 0;

  src.split(/\n/).forEach((line) => {
    lineno++;

    let state = 0;
    let quantity = 0;
    let set = '';
    let name = '';

    for (i = 0; i < line.length; i++) {
      let c = line.charAt(i);

      if (state == 0 && c == ' ') {
        // ignore whitespace
      } else if (state == 0 && c >= '0' && c <= '9') {
        state = 1;
        quantity = c - '0';
      } else if (state == 0 && c == '#') {
        state = 14;
      } else if (state == 1 && c >= '0' && c <= '9') {
        quantity = quantity * 10 + (c - '0');
      } else if (state == 1 && c == 'x') {
        state = 2;
      } else if (state == 1 && c == ' ') {
        state = 3;
      } else if (state == 2 && c == ' ') {
        state = 3;
      } else if (state == 3 && c == ' ') {
        // ignore whitespace
      } else if (state == 3 && ((c >= 'a' && c <= 'z')
                             || (c >= 'A' && c <= 'Z')
                             || (c >= '0' && c <= '9'))) {
        set += c;
        state = 4;
      } else if (state == 4 && ((c >= 'a' && c <= 'z')
                             || (c >= 'A' && c <= 'Z')
                             || (c >= '0' && c <= '9'))) {
        set += c;
      } else if (state == 4 && c == ' ') {
        state = 5;
      } else if (state == 5 && c == ' ') {
        // ignore whitespace
      } else if (state == 5 && ((c >= 'a' && c <= 'z')
                             || (c >= 'A' && c <= 'Z')
                             || (c >= '0' && c <= '9'))) {
        name += c;
        state = 6;
      } else if (state == 6 && c == '#') {
        state = 14;
      } else if (state == 6 && c == '|') {
        state = 7;
      } else if (state == 6) {
        name += c;
      } else if (state == 7 && c == ' ') {
        // ignore whitespace
      } else if (state == 7 && ((c >= 'a' && c <= 'z')
                             || (c >= 'A' && c <= 'Z')
                             || (c >= '0' && c <= '9'))) {
        //gvar += c;
        state = 8;
      } else if (state == 7 && c == '(') {
        state = 9;
      } else if (state == 7 && c == '#') {
        state = 14;
      } else if (state == 8 && c == ' ') {
        //gvars.push(gvar + c)
        //gvar = '';
        state = 7;
      } else if (state == 8 && ((c >= 'a' && c <= 'z')
                             || (c >= 'A' && c <= 'Z')
                             || (c >= '0' && c <= '9'))) {
        //gvar += c;
      } else if (state == 9 && ((c >= 'a' && c <= 'z')
                             || (c >= 'A' && c <= 'Z')
                             || (c >= '0' && c <= '9'))) {
        // lkey = c;
        state = 10;
      } else if (state == 10 && ((c >= 'a' && c <= 'z')
                              || (c >= 'A' && c <= 'Z')
                              || (c >= '0' && c <= '9'))) {
        // lkey += c;
      } else if (state == 10 && c == ':') {
        state = 11;
      } else if (state == 11 && c == ' ') {
        // ignore whitespace
      } else if (state == 11 && ((c >= 'a' && c <= 'z')
                              || (c >= 'A' && c <= 'Z')
                              || (c >= '0' && c <= '9'))) {
        //lval = c;
        state = 12;
      } else if (state == 12 && c == ')') {
        //lvars.push([lkey, lval]);
        state = 13;
      } else if (state == 12) {
        //lval += c;
      } else if (state == 13 && c == ' ') {
        state = 7;
      } else if (state == 13 && c == '#') {
        state = 14;
      } else if (state = 14) {
        // ignore comments
      } else {
        throw new Error('syntax error on line '+lineno.toString()+' '+JSON.stringify({s:state,c:c}));
      }
    }

    name = name.replace(/\s+$/, '');

    if (state == 8) {
      //gvars.push(gvar);
      state = 14;
    }

    if (state == 0 || state == 6 || state == 7 || state == 8 || state == 13 || state == 14) {
      if (quantity) {
        lines.push({
          line:     lineno,
          quantity: quantity,
          set:      set,
          oracle:   name
        });
      }
    } else {
      throw new Error('syntax error on line '+lineno.toString()+' '+JSON.stringify({s:state,eof:true}));
    }
  });

  return lines;
};

module.exports.CDIF = CDIF;
