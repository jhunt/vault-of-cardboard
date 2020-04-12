const fs     = require('fs'),
      chai   = require('chai'),
      expect = chai.expect,
      assert = chai.assert;

const cardboard = require('../index.js');

let $CARDS = JSON.parse(fs.readFileSync('test/data/cards.json'));

describe('CDIF Parser', () => {
  it('can handle empty input', () => {
    expect(cardboard.CDIF.parse('')).to.be.an('array').that.is.empty;
    expect(cardboard.CDIF.parse("\n")).to.be.an('array').that.is.empty;
    expect(cardboard.CDIF.parse("\n\n\n")).to.be.an('array').that.is.empty;
  });

  it('can handle input consisting solely of comments', () => {
    expect(cardboard.CDIF.parse('#')).to.be.an('array').that.is.empty;
    expect(cardboard.CDIF.parse(' #')).to.be.an('array').that.is.empty;
    expect(cardboard.CDIF.parse('# a comment')).to.be.an('array').that.is.empty;
    expect(cardboard.CDIF.parse(' # a comment')).to.be.an('array').that.is.empty;
    expect(cardboard.CDIF.parse("# comment\n# comment")).to.be.an('array').that.is.empty;
    expect(cardboard.CDIF.parse("\n\n# comment\n")).to.be.an('array').that.is.empty;
  });

  it('can handle simple, single-digit quantity lines', () => {
    expect(cardboard.CDIF.parse('1 MIR Island')).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 1,
      set      : 'MIR',
      oracle   : 'Island'
    }]);
  });

  it('can handle simple, single-digit quantity lines with the "x" suffix', () => {
    expect(cardboard.CDIF.parse('1x MIR Island')).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 1,
      set      : 'MIR',
      oracle   : 'Island'
    }]);
  });

  it('can handle simple, double-digit quantity lines suffix', () => {
    expect(cardboard.CDIF.parse("40x MIR Island\n40x MIR Mountain\n")).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 40,
      set      : 'MIR',
      oracle   : 'Island'
    }, {
      line     : 2,
      quantity : 40,
      set      : 'MIR',
      oracle   : 'Mountain'
    }]);
  });

  it('can handle interior whitespace between the quantity and the set code', () => {
    expect(cardboard.CDIF.parse("4x   MIR Island")).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 4,
      set      : 'MIR',
      oracle   : 'Island'
    }]);
  });

  it('can handle interior whitespace between the set code and oracle name', () => {
    expect(cardboard.CDIF.parse("4x MIR    Island")).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 4,
      set      : 'MIR',
      oracle   : 'Island'
    }]);
  });

  it('can handle lower-case set codes and oracle names', () => {
    expect(cardboard.CDIF.parse("4x mir island")).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 4,
      set      : 'mir',
      oracle   : 'island'
    }]);
  });

  it('can handle set codes with leading numeric digits', () => {
    expect(cardboard.CDIF.parse("18x 1AZ Island")).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 18,
      set      : '1AZ',
      oracle   : 'Island'
    }]);
  });

  it('can handle set codes with embedded numeric digits', () => {
    expect(cardboard.CDIF.parse("18x M19 Island")).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 18,
      set      : 'M19',
      oracle   : 'Island'
    }]);
  });

  it('can handle oracle names with embedded numeric digits', () => {
    expect(cardboard.CDIF.parse("8x UNS 2 4 The Show")).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 8,
      set      : 'UNS',
      oracle   : '2 4 The Show'
    }]);
  });

  it('can handle end-of-line comments', () => {
    expect(cardboard.CDIF.parse("1 MIR Island # the best one")).to.be.an('array').that.deep.equals([{
      line     : 1,
      quantity : 1,
      set      : 'MIR',
      oracle   : 'Island'
    }]);
  });

  it('can handle global variants', () => {
    expect(cardboard.CDIF.parse("1 MIR Island | foil SDCC 2legit2quit"))
      .to.be.an('array').that.deep.equals([{
        line     : 1,
        quantity : 1,
        set      : 'MIR',
        oracle   : 'Island'
      }]);
  });


  it('can handle global variants, with a trailing comment', () => {
    expect(cardboard.CDIF.parse("1 MIR Island | foil SDCC 2legit2quit     # super special"))
      .to.be.an('array').that.deep.equals([{
        line     : 1,
        quantity : 1,
        set      : 'MIR',
        oracle   : 'Island'
      }]);
  });

  it('can handle local variants', () => {
    expect(cardboard.CDIF.parse("1 MIR Island | (signed: SDCC '01) (MISPRINT: double)  (00: 007)"))
      .to.be.an('array').that.deep.equals([{
        line     : 1,
        quantity : 1,
        set      : 'MIR',
        oracle   : 'Island'
      }]);
  });

  it('can handle local variants, with a trailing comment', () => {
    expect(cardboard.CDIF.parse("1 MIR Island | (signed: SDCC '01) (MISPRINT: double)  (00: 007)# wtf"))
      .to.be.an('array').that.deep.equals([{
        line     : 1,
        quantity : 1,
        set      : 'MIR',
        oracle   : 'Island'
      }]);
  });

  // check that states 1, 2, 3, 4, 5, 9, 10, 11, 12 result in syntax errors
  it('detects syntax error from ending in state 1 (mid-number)', () =>
    expect(() => { cardboard.CDIF.parse('1') }).to.throw(/syntax/));
  it('detects syntax error from ending in state 2 (post-X-suffix)', () =>
    expect(() => { cardboard.CDIF.parse('1x') }).to.throw(/syntax/));
  it('detects syntax error from ending in state 3 (pre-set-code)', () =>
    expect(() => { cardboard.CDIF.parse('1x ') }).to.throw(/syntax/));
  it('detects syntax error from ending in state 4 (mid-set)', () =>
    expect(() => { cardboard.CDIF.parse('1x MIR') }).to.throw(/syntax/));
  it('detects syntax error from ending in state 5 (pre-oracle-name)', () =>
    expect(() => { cardboard.CDIF.parse('1x MIR ') }).to.throw(/syntax/));
  it('detects syntax error from ending in state 9 (pre-local-variant-key)', () =>
    expect(() => { cardboard.CDIF.parse('1x MIR Island | (') }).to.throw(/syntax/));
  it('detects syntax error from ending in state 10 (mid-local-variant-key)', () =>
    expect(() => { cardboard.CDIF.parse('1x MIR Island | (f') }).to.throw(/syntax/));
  it('detects syntax error from ending in state 11 (pre-local-variant-value)', () =>
    expect(() => { cardboard.CDIF.parse('1x MIR Island | (foo:') }).to.throw(/syntax/));
  it('detects syntax error from ending in state 12 (post-local-variant-value)', () =>
    expect(() => { cardboard.CDIF.parse('1x MIR Island | (foo:bar') }).to.throw(/syntax/));

});
