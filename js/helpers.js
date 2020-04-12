let manasym = classes => '<i class="ms ms-cost '+classes
                                                   .map(c => 'ms-'+c.toLowerCase())
                                                   .join(' ')+'"></i>';

module.exports = {
  symbolize: (s) => s.replace(/{(.+?)}/g, (_m, found, _offset, _s) =>
      manasym(
        found.toLowerCase().split('/')
          .map(sym => sym == 't' ? 'tap' : sym == 'q' ? 'untap' : sym))),

  colorize: (s) => s.split('').map(c => manasym([c])).join('')
};
