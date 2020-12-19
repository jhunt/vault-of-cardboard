let promises  = {}
let waitqueue = {}

let When = function(events, fn) {
  if (events instanceof Array) {
    if (events.length == 0) {
      Promise.resolve().then(fn)
      return
    }
    let first = events[events.length - 1],
        rest  = events.slice(0, -1)
    if (rest.length == 0) { return When(first, fn); }
    return When(rest, () => When(first, fn))
  }

  if (!promises[events]) {
    if (!(events in waitqueue)) { waitqueue[events] = []; }
    waitqueue[events].push(fn)

  } else {
    promises[events] = promises[events].then(fn); // FIXME no .catch()!
  }
}
When.trigger = function (ev) {
  promises[ev] = Promise.resolve()
  if (ev in waitqueue) {
    waitqueue[ev].forEach(fn => fn())
    delete waitqueue[ev]
  }
}

When.triggered = function (ev) {
  return (ev in promises)
}

When.clear = function (ev) {
  delete promises[ev]
}

export default {When}
