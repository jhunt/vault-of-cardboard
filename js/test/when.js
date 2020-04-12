const expect = require('chai').expect;

const cardboard = require('../index.js');

describe('When()', () => {
  let nextev = (() => {
    let id = 0;
    return () => {
      id += 1;
      let ev = ':event'+id.toString();
      expect(cardboard.When.triggered(ev), 'event "'+ev+'" should not already have triggered (test sanity)')
        .to.equal(false);
      return ev;
    }
  })();

  it('should immediately invoke callback if waiting on 0 events (edge case)', done => {
    cardboard.When([], done);
  });

  it('should handle waiting callbacks registered before an event trigger', done => {
    let ev = nextev();
    cardboard.When(ev, done);
    cardboard.When.trigger(ev);
  });

  it('should immediately invoke callbacks registered after event triggered', done => {
    let ev = nextev();
    cardboard.When.trigger(ev);
    cardboard.When(ev, done);
  });

  it('should handle multiple callbacks, post-registration', done => {
    let ev1 = nextev();
    let ev2 = nextev();
    cardboard.When.trigger(ev1);
    cardboard.When.trigger(ev2);

    var next;
    cardboard.When(ev1, () => { next = done; });
    cardboard.When(ev2, () => { next(); });
  });

  it('should handle multiple callbacks, pre-registration', done => {
    let ev1 = nextev();
    let ev2 = nextev();

    var next;
    cardboard.When(ev1, () => { next = done; });
    cardboard.When(ev2, () => { next(); });

    cardboard.When.trigger(ev1);
    cardboard.When.trigger(ev2);
  });

  it('should handle multiple callbacks, mid-registration', done => {
    let ev = nextev();

    var next;
    cardboard.When(ev, () => { next = done; });
    cardboard.When.trigger(ev);
    cardboard.When(ev, () => { next(); });
  });

  it('should handle callbacks waiting for multiple events', done => {
    let ev1 = nextev();
    let ev2 = nextev();

    cardboard.When([ev1, ev2], done);
    cardboard.When.trigger(ev1);
    cardboard.When.trigger(ev2);
  });

  it('should not care about order of events', done => {
    let ev1 = nextev();
    let ev2 = nextev();

    cardboard.When([ev2, ev1], done);
    cardboard.When.trigger(ev1);
    cardboard.When.trigger(ev2);
  });

  it('should allow events to be cleared', () => {
    let ev = nextev();
    cardboard.When.trigger(ev);
    expect(cardboard.When.triggered(ev)).to.equal(true);
    cardboard.When.clear(ev);
    expect(cardboard.When.triggered(ev)).to.equal(false);
  });
});
