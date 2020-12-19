let manasym = classes => '<i class="ms ms-cost '+classes
                                                   .map(c => 'ms-'+c.toLowerCase())
                                                   .join(' ')+'"></i>'

export default {
  symbolize: (s) => s.replace(/{(.+?)}/g, (_m, found /*, _offset, _s*/) =>
      manasym(
        found.toLowerCase().split('/')
          .map(sym => sym == 't' ? 'tap' : sym == 'q' ? 'untap' : sym))),

  shuffle: (cards) => {
    /* Fisher-Yates shuffle */
    for (let i = cards.length - 1; i > 0; i--) {
      let j = Math.floor(Math.random() * (i+1))
      let tmp = cards[i]
      cards[i] = cards[j]
      cards[j] = tmp
    }
    return cards
  }
}
