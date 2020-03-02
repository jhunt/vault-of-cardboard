;(function (window, document, undefined) {
  function Draft(options, cards) {
    /* FIXME: for now we ignore options and pretend its RIX */
    this.cards = { M:[], R:[], U:[], C:[], L:[] };

    for (var i = 0; i < cards.length; i++) {
      if (cards[i].type.match(/\bBasic Land\b/)) {
        this.cards.L.push(cards[i]);
        continue;
      }
      switch (cards[i].rarity) {
      case 'common':   this.cards.C.push(cards[i]); break;
      case 'uncommon': this.cards.U.push(cards[i]); break;
      case 'rare':     this.cards.R.push(cards[i]); break;
      case 'mythic':   this.cards.M.push(cards[i]); break;
      default: throw 'unrecognized rarity "'+cards[i].rarity+'"';
      }

      if (this.cards.M.length == 0) {
        this.cards.M = this.cards.R;
      }
    }
  }

  Draft.prototype.random = function (rarity) {
    var i = parseInt(Math.random() * this.cards[rarity].length);
    if (this.cards[rarity].length == 0) {
      console.log('unable to satisfy request for random card of rarity level "%s":', rarity, this.cards);
      return undefined;
    }
    return this.cards[rarity][i];
  };

  Draft.prototype.pack = function () {
    var cards = [];

    /* RARE slot */
    for (var n = 0; n < 1; n++) {
      if (Math.random() < (1 / 8.0)) {
        cards.push(this.random('M'));
      } else {
        cards.push(this.random('R'));
      }
    }

    /* UNCOMMON slots */
    for (var n = 0; n < 3; n++) {
      cards.push(this.random('U'));
    }

    /* COMMON slots */
    for (var n = 0; n < 10; n++) {
      cards.push(this.random('C'));
    }

    /* LAND slot */
    for (var n = 0; n < 1; n++) {
      cards.push(this.random('L'));
    }

    return cards;
  };

  window.Draft = Draft;
})(window, document);
