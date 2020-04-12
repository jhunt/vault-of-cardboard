let create = (options) => {
  if (options.set && !options.slots) {
    options.slots = [
      { name:  'rare',
        query: 'set:'+options.set+' and (=rare or =mythic)',
        count: 1 },
      { name:  'uncommon',
        query: 'set:'+options.set+' and =uncommon',
        count: 3 },
      { name:  'common',
        query: 'set:'+options.set+' and =common and !type:basic land',
        count: 10 },
      { name:  'common',
        query: 'set:'+(options.lands ? options.lands : options.set)+' and type:basic land',
        count: 1 }
    ];
  }
  return (draft) => options.slots.forEach(slot => draft.slot(slot.name, slot));
};

class Draft {
  constructor(vault, format) {
    this.vault = vault;
    this.slots = [];

    if (format) {
      if (typeof(format) === 'object') {
        create(format)(this);
      } else if (format in Draft.formats) {
        Draft.formats[format](this);
      } else {
        throw new Error('unknown draft format "'+format+'"');
      }
    }
  }

  slot(name, options) {
    this.slots.push({
      name: name,
      count: options.count || 1,
      cards: this.vault.search(options.query)
    });
    return this;
  }

  pack() {
    let cards = [];
    this.slots.forEach(slot => {
      let seen = {};
      for (let i = 0; i < slot.count; i++) {
        while (true) {
          let n = parseInt(Math.random() * slot.cards.length);
          if (!seen[n]) {
            cards.push(slot.cards[n]);
            seen[n] = true;
            break;
          }
        }
      }
    });
    return cards;
  }
}

Draft.formats = {};
Draft.format = function(name, options) {
  Draft.formats[name] = create(options);
};

module.exports.Draft = Draft;
