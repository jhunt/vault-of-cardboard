const fs     = require('fs'),
      expect = require('chai').expect;

const cardboard = require('../index.js');

let $CARDS = JSON.parse(fs.readFileSync('test/data/cards.json'));

describe('Vault.ingest()', () => {
  it('should handle a properly formatted input object', () => {
    let $v = new cardboard.Vault().ingest($CARDS);

    expect(!$v.has('MIR')).to.be.true; // MIR is a set code, not a card
    expect($v.has('51657034-2c30-40a2-a215-a00277f01642')).to.be.true; // Talruum Minotaur
    let card = $v.card('51657034-2c30-40a2-a215-a00277f01642');
    expect(card).to.deep.equal({
      id        : '51657034-2c30-40a2-a215-a00277f01642',
      oid       : 'c5661500-c48f-4f6d-bbe7-c8bd7118d862',
      flags     : '1EdlB$v',
      artist    : 'Pete Venters',
      layout    : 'normal',
      number    : '196',
      rarity    : 'common', // from flags
      image     : 'MIR/MIR-51657034-2c30-40a2-a215-a00277f01642.jpg',
      name      : 'Talruum Minotaur',
      type      : 'Creature — Minotaur Berserker',
      oracle    : 'Haste',
      cmc       : 4,
      cost      : '{2}{R}{R}',
      color     : 'R',
      price     : undefined,
      owned     : 0,
      flavor    : "Don't insult a Talruum unless your mount is swift. —Suq'Ata saying",
      power     : '3',
      toughness : '3',
      pt        : '3/3',
      art       : 'decd9cb4-ef84-4a79-b8bf-e3151535ccda',
      back      : '',
      set: {
        name: 'Mirage',
        code: 'MIR',
        total: 350
      }
    });
  });
});

describe('Color Identitiy Querying', () => {
  let $v = new cardboard.Vault().ingest($CARDS),
      $white    = $v.card('2a243bd7-af98-4e44-af6e-3b0b71d4837b'), // W:   Benevolent Unicorn
      $blue     = $v.card('854ba4e0-f6f3-4b6c-b6cb-ab2b93d64601'), // U:   Azimaet Drake
      $black    = $v.card('1b96810d-72d3-4dee-a29f-cdf85ea5ce6f'), // B:   Barbed-Back Wurm
      $red      = $v.card('486547cd-d2e7-4c46-9f7b-81c4267d65cc'), // R:   Burning Shield Askari
      $green    = $v.card('6c20edc3-5ad0-42c1-a5ec-3e680fb03297'), // G:   Brushwagg
      $none     = $v.card('046475e5-36d1-4b5f-af31-6df715c7a368'), //      Amber Prison

      $azorius  = $v.card('fb774a9b-d29f-4f41-9fb8-a0189205e16f'), // WU:  Prismatic Boon
      $dimir    = $v.card('176d625f-1410-4ad6-a279-9a184fac6507'), // UB:  Spatial Binding
      $rakdos   = $v.card('312bbc1b-4c2a-44c1-8e62-c0f94fd2ba8e'), // BR:  Phyrexian Purge
      $gruul    = $v.card('59fb9591-399a-4196-a52d-f2954d287a10'), // RG:  Jungle Troll
      $selesnya = $v.card('5fe8a5b8-1a87-46f5-920f-fbbb05bfd563'), // WG:  Vitalizing Cascade

      $orzhov   = $v.card('ef9d2c05-0d2f-4d02-aef3-e1078d78e5ff'), // WB:  Circle of Despair
      $izzet    = $v.card('50d4468b-f7de-44fe-898a-4125d26d242f'), // UR:  Frenetic Efreet
      $golgari  = $v.card('c9bef70b-61c7-4df5-b4df-09cd6ab2015c'), // BG:  Cadaverous Bloom
      $boros    = $v.card('711f4cff-0256-44b2-a2fe-1cae6e9edb2b'), // WR:  Energy Bolt
      $simic    = $v.card('760b6703-ac92-45f6-8c32-60f760eba866'), // UG:  Malignant Growth

      $bant     = $v.card('a9319039-db2f-47bf-9ef0-8d3a381d54fb'), // WUG: Civic Guildmage
      $esper    = $v.card('4a9304e1-f403-404d-9fe9-169da75e0d62'), // WUB: Shaper Guildmage
      $grixis   = $v.card('ba3fc11e-db36-430c-920b-31195913c16a'), // UBR: Shadow Guildmage
      $jund     = $v.card('e999fdc3-9269-44d7-9015-e16f5e5b73eb'), // BRG: Armorer Guildmage
      $naya     = $v.card('3f495b27-3eed-4962-b69a-b86f9fc6a9a7'), // WRG: Granger Guildmage

      $all   = [
        $white, $blue, $black, $red, $green, $none,
        $azorius, $dimir, $rakdos, $gruul, $selesnya,
        $orzhov, $izzet, $golgari, $boros, $simic,
        $bant, $esper, $grixis, $jund, $naya,
        //$abzan, $jeskai, $sultai, $mardu, $temur
      ],
      except = (x) => {
        let l = [];
        $all.forEach(y => { if (x != y) { l.push(y); } });
        return l;
      };

  let should_match = (q, summary, $match) =>
    it('should recognize '+q+' as '+summary, () => {
      let $q = cardboard.Query.parse(q);
      expect($q.match($match)).to.be.true;
      except($match).forEach($other => expect($q.match($other)).to.be.false)
    });

  should_match('@none',  'colorless',  $none);

  should_match('@white', 'mono-white', $white);
  should_match('@blue',  'mono-blue',  $blue);
  should_match('@black', 'mono-black', $black);
  should_match('@red',   'mono-red',   $red);
  should_match('@green', 'mono-green', $green);

  should_match('@azorius',  'white + blue',  $azorius);
  should_match('@dimir',    'blue + black',  $dimir);
  should_match('@rakdos',   'black + red',   $rakdos);
  should_match('@gruul',    'red + green',   $gruul);
  should_match('@selesnya', 'white + green', $selesnya);

  should_match('@orzhov',  'white + black', $orzhov);
  should_match('@izzet',   'red + blue',    $izzet);
  should_match('@golgari', 'green + black', $golgari);
  should_match('@boros',   'white + red',   $boros);
  should_match('@simic',   'blue + green',  $simic);

  should_match('@bant',   'white + blue + green', $bant);
  should_match('@esper',  'white + blue + black', $esper);
  should_match('@grixis', 'blue + black + red',   $grixis);
  should_match('@jund',   'black + red + green',  $jund);
  should_match('@naya',   'white + red + green',  $naya);

  //should_match('@dimir', '', $dimir);
});

describe('Vault.search()', () => {
  let $v = new cardboard.Vault().ingest($CARDS);

  let should_find_total = (q, n) =>
    it('should find '+n.toString()+' card(s) for ('+q+')',
      () => expect($v.search(q, 1000).length).to.be.equal(n));

  let should_find_nothing = (q) => should_find_total(q, 0);

  let should_find = (q, card) =>
    it('should find card "'+card+'" via ('+q+')', () => {
      let r = $v.search(q, 1000);
      expect(r.length, 'no cards found via search ('+q+')!').to.be.above(0);
      for (var i = 0; i < r.length; i++) {
        if (r[i].name == card) {
          expect(true).to.be.true;
          return;
        }
      }
      expect.fail('did not find "'+card+'" in '+r.length.toString()+' results...');
    });

  should_find_total('set:MIR', 350);
  should_find_total('set:MIR and Talruum Minotaur', 1);
  should_find('set:MIR and Talruum Minotaur', 'Talruum Minotaur');

  should_find_total('set:MIR and type:artifact', 33);
  should_find('set:MIR and type:artifact', 'Amber Prison');

  should_find_total('set:MIR and Island', 4);
  should_find_total('set:MIR and Island and unique:art', 4);
  should_find_total('set:MIR and Island and unique', 1);

  'activate discard exile tap untap sacrifice'.split(/ /).forEach(k =>
    should_find_nothing('!oracle:.* and '+k+':'));

  should_find('set:MIR and =common', 'Crash of Rhinos');
  should_find('set:MIR and =c', 'Crash of Rhinos');

  should_find('set:MIR and =uncommon', 'Bad River');
  should_find('set:MIR and =u', 'Bad River');

  should_find('set:MIR and =rare', 'Brushwagg');
  should_find('set:MIR and =r', 'Brushwagg');
});

describe('Vault.clarify()', () => {
  let $v = new cardboard.Vault().ingest($CARDS);
  let included = (results, set, card) => {
    for (var i = 0; i < results.length; i++) {
      if (results[i].set == set && results[i].name == card) {
        return true;
      }
    }
    return false;
  };

  it('should be able to clarify an exact match', () => {
    let card = $v.clarify('MIR', 'Bone Mask');
    expect(card).to.include({name: 'Bone Mask'});
  });

  it('should be able to clarify what you meant by "Island"', () => {
    let clarifications = $v.clarify(undefined, "Island");
    let best = clarifications[0];
    expect(best).to.include({
      set:  'MIR',
      name: 'Island',
      type: 'global'
    });
  });

  it('should be able to clarify what you meant by "[VIS] Island"', () => {
    let clarifications = $v.clarify('VIS', "Island");
    expect(clarifications).to.be.an('array');

    let best = clarifications[0];
    expect(best).to.include({
      set:  'MIR',
      name: 'Island',
      type: 'global'
    });
  });

  it('should be able to clarify what you meant by "[MIR] Islan"', () => {
    let clarifications = $v.clarify('MIR', "Islan");
    expect(clarifications).to.be.an('array');

    let best = clarifications[0];
    expect(best).to.include({
      set:  'MIR',
      name: 'Island',
      type: 'in-set'
    });
  });

  it('should be able to clarify what you meant by "Talrun Minoatur"', () => {
    let clarifications = $v.clarify('', "Talrun Minoatur");
    expect(clarifications).to.be.an('array');

    let best = clarifications[0];
    expect(best).to.include({
      set:  'MIR',
      name: 'Talruum Minotaur',
      type: 'global'
    });
  });

  it('should be able to clarify what you meant by "Burning-SHIELD Askari"', () => {
    let clarifications = $v.clarify('', "Burning-SHIELD Askari");
    expect(clarifications).to.be.an('array');

    let best = clarifications[0];
    expect(best).to.include({
      set:  'MIR',
      name: 'Burning Shield Askari',
      type: 'global'
    });
  });
});

describe('Vault.load_collection()', () => {
  let $v = new cardboard.Vault().ingest($CARDS);
  let should_own = (qty, name) => {
    let l = $v.search('owned and '+name);
    expect(l).to.be.an('array');
    expect(l.length).to.equal(qty == 0 ? 0 : 1);
    if (qty > 0) {
      expect(l[0]).to.include({ name: name, owned: qty });
    }
  };

  let base = [
    [11, {pid: "cbac1d27-15e2-4e2f-82ab-625a16e096cb", "var": []}], // 11 MIR Enlightened Tutor
    [21, {pid: "5d98101f-e32a-4a4a-a649-faa920d111ee", "var": []}], // 21 MIR Mystical Tutor
    [31, {pid: "f00115bc-b551-4bf5-a121-bebb37201575", "var": []}], // 31 MIR Worldly Tutor
  ];
  let patches = [
    [], // empty patch

    // 1 MIR Enlightened Tutor  # cbac
    // 2 MIR Mystical Tutor     # 5d98
    // 3 MIR Wordly Tutor       # f001
    [{id: "cbac1d27-15e2-4e2f-82ab-625a16e096cb", quantity: 1,  gvars: [], lvars: []},
     {id: "5d98101f-e32a-4a4a-a649-faa920d111ee", quantity: 2,  gvars: [], lvars: []},
     {id: "f00115bc-b551-4bf5-a121-bebb37201575", quantity: 3,  gvars: [], lvars: []}],

    // 10 MIR Plains    # 81bb
    // 10 MIR Island    # a39f
    // 10 MIR Swamp     # 5083
    // 10 MIR Mountain  # 1cef
    // 10 MIR Forest    # 95df
    [{id: "81bbbf38-5d1a-4013-aff9-6167709897f0", quantity: 10, gvars: [], lvars: []},
     {id: "a39fc1e0-caf0-4cfa-bbf2-fea7ca32c00d", quantity: 10, gvars: [], lvars: []},
     {id: "5083de34-d127-45df-9252-ff09b5cf8b47", quantity: 10, gvars: [], lvars: []},
     {id: "1cef9230-34fa-496f-8835-5dfaac627f70", quantity: 10, gvars: [], lvars: []},
     {id: "95dfef30-acca-4b15-a05e-d33289055218", quantity: 10, gvars: [], lvars: []}],
  ];

  should_own(0, 'Enlightened Tutor');
  should_own(0, 'Mystical Tutor');
  should_own(0, 'Worldly Tutor');

  should_own(0, 'Plains');
  should_own(0, 'Island');
  should_own(0, 'Swamp');
  should_own(0, 'Mountain');
  should_own(0, 'Forest');

  $v.load_collection(base, patches);
  should_own(12, 'Enlightened Tutor');
  should_own(23, 'Mystical Tutor');
  should_own(34, 'Worldly Tutor');

  should_own(10, 'Plains');
  should_own(10, 'Island');
  should_own(10, 'Swamp');
  should_own(10, 'Mountain');
  should_own(10, 'Forest');

  $v.no_collection();
  should_own(0, 'Enlightened Tutor');
  should_own(0, 'Mystical Tutor');
  should_own(0, 'Worldly Tutor');

  should_own(0, 'Plains');
  should_own(0, 'Island');
  should_own(0, 'Swamp');
  should_own(0, 'Mountain');
  should_own(0, 'Forest');
});

describe('Vault.when()', () => {
  var $EMPTY = {
    cards: [],
    sets:  {}
  };

  it('should specify callback names in exports', () => {
    expect(cardboard.CardsLoaded).to.equal('cards-loaded');
    expect(cardboard.CollectionLoaded).to.equal('collection-loaded');
  });

  it('should handle awaiting callbacks registered before card ingestion', done => {
    let $v = new cardboard.Vault().when('foo', done);
    $v.trigger('foo');
  });

  it('should immediately invoke callbacks registered after card ingestion', done => {
    let $v = new cardboard.Vault().trigger('foo');
    $v.when('foo', done);
  });

  it('should handle multiple callbacks, post-registration', done => {
    let $v = new cardboard.Vault().trigger('foo');
    var next;
    $v.when('foo', () => { next = done; });
    $v.when('foo', () => { next(); });
  });

  it('should handle multiple callbacks, pre-registration', done => {
    let $v = new cardboard.Vault();
    var next;
    $v.when('foo', () => { next = done; });
    $v.when('foo', () => { next(); });
    $v.trigger('foo');
  });

  it('should handle multiple callbacks, mid-registration', done => {
    let $v = new cardboard.Vault();
    var next;
    $v.when('foo', () => { next = done; });
    $v.trigger('foo');
    $v.when('foo', () => { next(); });
  });

  it('should handle the "cards loaded" callback', done => {
    new cardboard.Vault().when(cardboard.CardsLoaded, done).ingest($EMPTY);
  });

  it('should handle the "collection loaded" callback', done => {
    new cardboard.Vault().when(cardboard.CollectionLoaded, done).load_collection([], []);
  });

  it('should handle the "collection loaded" callback, even if there is no collection', done => {
    new cardboard.Vault().when(cardboard.CollectionLoaded, done).no_collection();
  });

  it('should handle callbacks waiting for multiple events', done => {
    let $v = new cardboard.Vault();
    $v.when([cardboard.CardsLoaded, cardboard.CollectionLoaded], done);
    $v.ingest($EMPTY);
    $v.no_collection();
  });

  it('should not care what order event list is specified in', done => {
    let $v = new cardboard.Vault();
    $v.when([cardboard.CollectionLoaded, cardboard.CardsLoaded], done);
    $v.ingest($EMPTY);
    $v.no_collection();
  });
});
