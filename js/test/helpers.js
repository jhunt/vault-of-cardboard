const fs     = require('fs'),
      expect = require('chai').expect;

const cardboard = require('../index.js');

describe('symbolize()', () => {
  let sym = (src, want) => {
    return () => expect(cardboard.symbolize(src)).to.equal(want);
  };

  it('should handle mana symbols',     sym('{R}', '<i class="ms ms-cost ms-r"></i>'));
  it('should handle the tap symbol',   sym('{T}', '<i class="ms ms-cost ms-tap"></i>'));
  it('should handle the untap symbol', sym('{Q}', '<i class="ms ms-cost ms-untap"></i>'));
  it('should handle multiple symbols', sym('{2}{Q}',
    '<i class="ms ms-cost ms-2"></i>'+
    '<i class="ms ms-cost ms-untap"></i>'));
  it('should handle embedded non-symbols', sym('Pay {2}{U}{R}, Discard: Win Game',
    'Pay <i class="ms ms-cost ms-2"></i>'+
    '<i class="ms ms-cost ms-u"></i>'+
    '<i class="ms ms-cost ms-r"></i>'+
    ', Discard: Win Game'));

  it('should handle multi-line inputs', sym("{2}{W}: +1\n{2}{B}: -1\n",
    '<i class="ms ms-cost ms-2"></i><i class="ms ms-cost ms-w"></i>: +1'+"\n"+
    '<i class="ms ms-cost ms-2"></i><i class="ms ms-cost ms-b"></i>: -1'+"\n"));
});

describe('colorize()', () => {
  let color = (src, want) => {
    return () => expect(cardboard.colorize(src)).to.equal(want);
  };

  it('should handle white',     color('W', '<i class="ms ms-cost ms-w"></i>'));
  it('should handle blue',      color('U', '<i class="ms ms-cost ms-u"></i>'));
  it('should handle black',     color('B', '<i class="ms ms-cost ms-b"></i>'));
  it('should handle red',       color('R', '<i class="ms ms-cost ms-r"></i>'));
  it('should handle green',     color('G', '<i class="ms ms-cost ms-g"></i>'));
  it('should handle colorless', color('C', '<i class="ms ms-cost ms-c"></i>'));

  it('should handle izzet', color('UR',
    '<i class="ms ms-cost ms-u"></i>'+
    '<i class="ms ms-cost ms-r"></i>'));

  it('should handle WUBRG', color('WUBRG',
    '<i class="ms ms-cost ms-w"></i>'+
    '<i class="ms ms-cost ms-u"></i>'+
    '<i class="ms ms-cost ms-b"></i>'+
    '<i class="ms ms-cost ms-r"></i>'+
    '<i class="ms ms-cost ms-g"></i>'));
});
