const fs     = require('fs'),
      expect = require('chai').expect;

const cardboard = require('../index.js');

let $CARDS = JSON.parse(fs.readFileSync('test/data/cards.json'));

describe('Draft', () => {
  let ntimes = (n, fn) => Array.from({ length: n }).forEach(fn);
  let N = 1000;

  let $v = new cardboard.Vault().ingest($CARDS);

  describe('with a single slot', () => {
    let draft = new cardboard.Draft($v)
      .slot('basic', {
        query:  'Guildmage',
        count:  2,
        unique: true,
      });

    let $civic   = $v.find('MIR', 'Civic Guildmage');
    let $shaper  = $v.find('MIR', 'Shaper Guildmage');
    let $shadow  = $v.find('MIR', 'Shadow Guildmage');
    let $armorer = $v.find('MIR', 'Armorer Guildmage');
    let $granger = $v.find('MIR', 'Granger Guildmage');

    it('should have a good test base (test sanity)', () => {
      expect($civic).to.not.be.null;
      expect($shaper).to.not.be.null;
      expect($shadow).to.not.be.null;
      expect($armorer).to.not.be.null;
      expect($granger).to.not.be.null;
    });

    it('should return a pack of 2 cards', () => {
      let cards = draft.pack();
      expect(cards).to.be.an('array');
      expect(cards.length).to.be.equal(2);
      expect(cards[0]).to.be.not.null;
      expect(cards[1]).to.be.not.null;
    });

    it('should always return a pack of 2 randomly selected cards', () => {
      dist = {};
      ntimes(N, () => {
        draft.pack().forEach(card => dist[card.id] = (dist[card.id] || 0) + 1);
      });
      let a = N * 2 * (0.20 - 0.10);
      let b = N * 2 * (0.20 + 0.10);
      console.dir(dist);
      expect(dist[$civic.id]   || 0).to.be.within(a,b, 'Civic Guildmage should be ~20%');
      expect(dist[$shaper.id]  || 0).to.be.within(a,b, 'Shaper Guildmage should be ~20%');
      expect(dist[$shadow.id]  || 0).to.be.within(a,b, 'Shadow Guildmage should be ~20%');
      expect(dist[$armorer.id] || 0).to.be.within(a,b, 'Armorer Guildmage should be ~20%');
      expect(dist[$granger.id] || 0).to.be.within(a,b, 'Granger Guildmage should be ~20%');
    });

    it('should never repeat a card in the same pack', () => {
      let repeats = 0;
      ntimes(N, () => {
        let dupes = 0, seen = {};
        draft.pack().forEach(card => seen[card.id] ? dupes++ : seen[card.id] = true);
        if (dupes > 0) { repeats++; }
      });
      expect(repeats).to.be.equal(0, "should have no repeats in any packs");
    });
  });

  describe('manually-specified formats', () => {
    cardboard.Draft.format('MIR', {
      slots: [
        { name:  'rare',
          query: 'set:MIR and =rare',
          count: 1 },
        { name:  'uncommon',
          query: 'set:MIR and =uncommon',
          count: 3 },
        { name:  'common',
          query: 'set:MIR and =common and !type:basic land',
          count: 10 },
        { name:  'land',
          query: 'set:MIR and type:basic land',
          count: 1 }
      ]
    });

    let pack = new cardboard.Draft($v, 'MIR').pack();
    expect(pack.length).to.equal(15);

    let seen = {};
    let i = 0;
    expect(pack[i].set.code).to.equal('MIR');
    expect(pack[i].rarity).to.equal('rare');
    seen[pack[i].id] = true;

    ntimes(3, () => {
      i++;
      expect(seen[pack[i].id]).to.be.undefined;
      expect(pack[i].set.code).to.equal('MIR');
      expect(pack[i].rarity).to.equal('uncommon');
      seen[pack[i].id] = true;
    });
    ntimes(10, () => {
      i++;
      expect(seen[pack[i].id]).to.be.undefined;
      expect(pack[i].set.code).to.equal('MIR');
      expect(pack[i].rarity).to.equal('common');
      expect(pack[i].type).to.not.match(/basic land/i);
      seen[pack[i].id] = true;
    });
    i++;
    expect(seen[pack[i].id]).to.be.undefined;
    expect(pack[i].set.code).to.equal('MIR');
    expect(pack[i].type).to.match(/basic land/i);
  });

  describe('inferred formats', () => {
    cardboard.Draft.format('VIS', {
      set:   'VIS',
      lands: 'MIR',
    });

    let pack = new cardboard.Draft($v, 'VIS').pack();
    expect(pack.length).to.equal(15);

    let seen = {};
    let i = 0;
    expect(pack[i].set.code).to.equal('VIS');
    expect(pack[i].rarity).to.equal('rare');
    seen[pack[i].id] = true;

    ntimes(3, () => {
      i++;
      expect(seen[pack[i].id]).to.be.undefined;
      expect(pack[i].set.code).to.equal('VIS');
      expect(pack[i].rarity).to.equal('uncommon');
      seen[pack[i].id] = true;
    });
    ntimes(10, () => {
      i++;
      expect(seen[pack[i].id]).to.be.undefined;
      expect(pack[i].set.code).to.equal('VIS');
      expect(pack[i].rarity).to.equal('common');
      expect(pack[i].type).to.not.match(/basic land/i);
      seen[pack[i].id] = true;
    });
    i++;
    expect(seen[pack[i].id]).to.be.undefined;
    expect(pack[i].set.code).to.equal('MIR');
    expect(pack[i].type).to.match(/basic land/i);
  });
});
