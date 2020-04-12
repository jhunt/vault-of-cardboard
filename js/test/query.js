const expect = require('chai').expect;

const cardboard = require('../index.js');

describe('Cardboard Query Parser', () => {
  let test = (q, want) => { return () => expect(cardboard.Query.parse(q).toString()).to.equal(want) };

  it('should infer case-insensitive name pattern-matching by default',
    test('foo', '(NAME /\\bfoo\\b/i)'));

  it('should infer case-sensitive name pattern-matching when quoted',
    test('"foo"', '(NAME /\\bfoo\\b/)'));

  it('should treat multiple whitespace-separated keywords as a single name match',
    test('a b c', '(NAME /\\ba b c\\b/i)'));

  "WUBRG".split('').forEach(c =>
    it('should parse references to single-color {'+c+'} identity',
      test('@'+c, '(COLOR '+c+')'))
  );

  it('should handle rarity-based queries',
    test('=uncommon', '(RARITY 2)'));

  it('should handle set membership queries',
    test('set:MIR', '(SET MIR)'));

  it('should handle uniqueness constraints',
    test('unique:art', '(UNIQUE art)'));

  [
    'type',
    'oracle',
    'flavor',
    'oracle',
    'activate',
    'discard',
    'tap',
    'untap'
  ].forEach(x =>
    it('should handle '+x+' queries (as partial-match)',
      test(x+':foo', '('+x.toUpperCase()+' /\\bfoo\\b/i)')));
  [
    'layout',
  ].forEach(x =>
    it('should handle '+x+' queries (as keyword-match)',
      test(x+':foo', '('+x.toUpperCase()+' foo)')));

  [
    'equip',
    'own',
    'usd',
    'cmc',
    'p',
    't',
    'cpt',
    'ptr'
  ].forEach(x => {
    it('should handle '+x+':2 queries as numeric (equality)',
      test(x+':2', '('+x.toUpperCase()+' 2)'));
    it('should handle '+x+':2+ queries as numeric (greater than or equal to)',
      test(x+':2+', '('+x.toUpperCase()+' 2+)'));
    it('should handle '+x+':>2 queries as numeric (greater than)',
      test(x+':>2', '('+x.toUpperCase()+' >2)'));
    it('should handle '+x+':>=2 queries as numeric (greater than or equal to)',
      test(x+':>=2', '('+x.toUpperCase()+' >=2)'));
    it('should handle '+x+':<2 queries as numeric (less than)',
      test(x+':<2', '('+x.toUpperCase()+' <2)'));
    it('should handle '+x+':<=2 queries as numeric (less than or equal to)',
      test(x+':<=2', '('+x.toUpperCase()+' <=2)'));
  });

  [
    'fullart',
    'oversized',
    'variant',
    'spotlight',
    'reserved',
    'reprint'
  ].forEach(x => {
    it('should handle '+x+':yes queries',
      test(x+':yes', '('+x.toUpperCase()+' yes)'));
    it('should handle '+x+':y queries',
      test(x+':y', '('+x.toUpperCase()+' yes)'));
    it('should handle '+x+':no queries',
      test(x+':no', '('+x.toUpperCase()+' no)'));
    it('should handle '+x+':n queries',
      test(x+':n', '('+x.toUpperCase()+' no)'));
    it('should handle '+x+':1 queries',
      test(x+':1', '('+x.toUpperCase()+' yes)'));
  });

  it('should handle negation of queries',
    test('!cmc:2', '(NOT (CMC 2))'));

  it('should handle logical AND composition',
    test('own:2 and cmc:2', '(AND (OWN 2) (CMC 2))'));

  it('should handle logical OR composition',
    test('own:2 or cmc:2', '(OR (OWN 2) (CMC 2))'));

  it('should handle un-grouped AND + OR branching',
    test('@R or @B and @U', '(AND (OR (COLOR R) (COLOR B)) (COLOR U))'));

  it('should handle explicitly-grouped AND + OR branching',
    test('@R or (@B and @U)', '(OR (COLOR R) (AND (COLOR B) (COLOR U)))'));

  it('should handle grouped and negated sub-queries',
    test('@R or (!@U and !@B)', '(OR (COLOR R) (AND (NOT (COLOR U)) (NOT (COLOR B))))'));

  it('should handle brawl legality queries',      test('legal:brawl',      '(LEGAL B)'));
  it('should handle edh legality queries',        test('legal:edh',        '(LEGAL E)'));
  it('should handle commander legality queries',  test('legal:commander',  '(LEGAL E)'));
  it('should handle duel legality queries',       test('legal:duel',       '(LEGAL d)'));
  it('should handle frontier legality queries',   test('legal:frontier',   '(LEGAL j)'));
  it('should handle future legality queries',     test('legal:future',     '(LEGAL f)'));
  it('should handle historic legality queries',   test('legal:historic',   '(LEGAL h)'));
  it('should handle legacy legality queries',     test('legal:legacy',     '(LEGAL l)'));
  it('should handle modern legality queries',     test('legal:modern',     '(LEGAL m)'));
  it('should handle old-school legality queries', test('legal:old-school', '(LEGAL o)'));
  it('should handle pauper legality queries',     test('legal:pauper',     '(LEGAL P)'));
  it('should handle penny legality queries',      test('legal:penny',      '(LEGAL $)'));
  it('should handle pioneer legality queries',    test('legal:pioneer',    '(LEGAL p)'));
  it('should handle standard legality queries',   test('legal:standard',   '(LEGAL s)'));
  it('should handle vintage legality queries',    test('legal:vintage',    '(LEGAL v)'));
});

describe('Individual Card Querying', () => {
  let $c = {
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
    price     : 1.15, // price-fixing
    owned     : 4,
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
  };
  let should_match = (m,q) => it('should match '+m+' ('+q+')',
    () => expect(cardboard.Query.parse(q).match($c)).to.be.true);

  should_match('based on name predicate',      'talruum');
  should_match('based on oracle predicate',    '+haste');
  should_match('based on flavor predicate',    'flavor:insult');
  should_match('based on set predicate',       'set:MIR');
  should_match('based on type predicate',      'type:creature');
  should_match('based on artist predicate',    'artist:Pete');
  should_match('based on color predicate',     '@R');
  should_match('based on cmc predicate',       'cmc:4');
  should_match('based on layout predicate',    'layout:normal');
  should_match('based on rarity predicate',    '=common');
  should_match('based on power predicate',     'p:3');
  should_match('based on toughness predicate', 't:3');
  should_match('based on total p/t',           'pt:3/3');
  should_match('based on combined p+t',        'cpt:6');
  should_match('based on ratio of p:t',        'ptr:1');
  should_match('based on legality predicate',  'legal:vintage');

  should_match('based on oversized predicate', 'oversized:no');
  should_match('based on fullart predicate',   'fullart:no');
  should_match('based on spotlight predicate', 'spotlight:no');
  should_match('based on variant predicate',   'variant:no');
  should_match('based on reserved predicate',  'reserved:no');
  should_match('based on reprint predicate',   'reprint:no');

  should_match('based on (negated) reprint predicate', '!reprint');

  should_match('based on usd predicate',       'usd:1.15');
  should_match('based on usd predicate',       'usd:1+');
  should_match('based on usd predicate',       'usd:>1');
  should_match('based on usd predicate',       'usd:<2');
  should_match('based on usd predicate',       'usd:<=2');

  should_match('based on owned predicate',     'own:4');

  should_match('based on negated equip predicate', '!equip:2');

  should_match('based on a complicated query',
    'Talruum and Minotaur and (@U or @R) and !=mythic');

  it('should allow match cards based on equip costs', () => {
    let c = { oracle: 'Equip {2}.' };
    expect(cardboard.Query.parse('equip:2').match(c)).to.be.true;
    expect(cardboard.Query.parse('equip:1+').match(c)).to.be.true;
    expect(cardboard.Query.parse('equip:<=3').match(c)).to.be.true;
  });

  it('should allow match cards based on {T} activation', () => {
    let c = { oracle: '{T}: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('tap:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('tap:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('tap:').match(c)).to.be.true;
    expect(cardboard.Query.parse('tap: and +do the thing').match(c)).to.be.true;
  });

  it('should allow match cards based on {Q} activation', () => {
    let c = { oracle: '{Q}: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('untap:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('untap:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('untap:').match(c)).to.be.true;
    expect(cardboard.Query.parse('untap: and +do the thing').match(c)).to.be.true;
  });

  it('should allow match cards based on "sacrifice" activation', () => {
    let c = { oracle: 'Sacrifice a creature: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('sacrifice:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('sacrifice:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('sacrifice:').match(c)).to.be.true;
    expect(cardboard.Query.parse('sacrifice: and +do the thing').match(c)).to.be.true;
  });

  it('should allow match cards based on "discard" activation', () => {
    let c = { oracle: 'Discard a card: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('discard:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('discard:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('discard:').match(c)).to.be.true;
    expect(cardboard.Query.parse('discard: and +do the thing').match(c)).to.be.true;
  });

  it('should allow match cards based on "exile" activation', () => {
    let c = { oracle: 'Exile a token: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('exile:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('exile:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('exile:').match(c)).to.be.true;
    expect(cardboard.Query.parse('exile: and +do the thing').match(c)).to.be.true;
  });

  it('should allow match cards based on "pay" activation', () => {
    let c = { oracle: 'Exile a token: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

//    expect(cardboard.Query.parse('pay:do the thing').match(c)).to.be.true;
//    expect(cardboard.Query.parse('pay:"Do the thing"').match(c)).to.be.true;
//    expect(cardboard.Query.parse('pay:').match(c)).to.be.true;
//    expect(cardboard.Query.parse('pay: and +do the thing').match(c)).to.be.true;
  });
});
