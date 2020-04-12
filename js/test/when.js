const chai   = require('chai'),
      expect = chai.expect,
      assert = chai.assert;

const When = require('../when.js').When;

describe('When()', () => {
  let nextev = (() => {
    let id = 0;
    return () => {
      id += 1;
      let ev = ':event'+id.toString();
      expect(When.triggered(ev), 'event "'+ev+'" should not already have triggered (test sanity)')
        .to.equal(false);
      return ev;
    }
  })();

  it('should immediately invoke callback if waiting on 0 events (edge case)', done => {
    When([], done);
  });

  it('should handle waiting callbacks registered before an event trigger', done => {
    let ev = nextev();
    When(ev, done);
    When.trigger(ev);
  });

  it('should immediately invoke callbacks registered after event triggered', done => {
    let ev = nextev();
    When.trigger(ev);
    When(ev, done);
  });

  it('should handle multiple callbacks, post-registration', done => {
    let ev1 = nextev();
    let ev2 = nextev();
    When.trigger(ev1);
    When.trigger(ev2);

    var next;
    When(ev1, () => { next = done; });
    When(ev2, () => { next(); });
  });

  it('should handle multiple callbacks, pre-registration', done => {
    let ev1 = nextev();
    let ev2 = nextev();

    var next;
    When(ev1, () => { next = done; });
    When(ev2, () => { next(); });

    When.trigger(ev1);
    When.trigger(ev2);
  });

  it('should handle multiple callbacks, mid-registration', done => {
    let ev = nextev();

    var next;
    When(ev, () => { next = done; });
    When.trigger(ev);
    When(ev, () => { next(); });
  });

  it('should handle callbacks waiting for multiple events', done => {
    let ev1 = nextev();
    let ev2 = nextev();

    When([ev1, ev2], done);
    When.trigger(ev1);
    When.trigger(ev2);
  });

  it('should not care about order of events', done => {
    let ev1 = nextev();
    let ev2 = nextev();

    When([ev2, ev1], done);
    When.trigger(ev1);
    When.trigger(ev2);
  });

  it('should allow events to be cleared', () => {
    let ev = nextev();
    When.trigger(ev);
    expect(When.triggered(ev)).to.equal(true);
    When.clear(ev);
    expect(When.triggered(ev)).to.equal(false);
  });
});
