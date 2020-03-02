;(function (window, document, undefined) {
  var TabletopAlpha = function (deck, e) {
    var self = this;
    $.ajax({
      type: 'GET',
      url:  '/v1/decks/'+deck+'/cards.json',
      success: function (cards) {
        self.mount(e);

        /* master card "database" */
        self.cards = { all: [] };
        for (var i = 0; i < cards.cards.length; i++) {
          var c = {
            id:   cards.cards[i],
            card: cards.oracle[cards.cards[i].replace(/\/.*/, '')]
          };
          self.cards[cards.cards[i]] = c;
          self.cards.all.push(c);
        }

        /* set up the library */
        self.library = shuffle(self.cards.all);
        e.find('.library').removeClass('empty');

        /* draw opening hand */
        var h = e.find('.hand');
        for (var i = 0; i < 7; i++) {
          h.append(self.watch($(self.card(self.library.shift()))));
        }
        self.check();
      }
    });
  };

  /* shuffle(cards) - shuffle cards in a deck / pile {{{ */
  var shuffle = function (cards) {
    var i, j, card; /* Fisher-Yates */
    for (i = cards.length - 1; i > 0; i--) {
      j = Math.floor(Math.random() * (i + 1));
      card = cards[i];
      cards[i] = cards[j];
      cards[j] = card;
    }
    return cards;
  };
  /* }}} */
  window.shuffle = shuffle;

   /* t.watch(card) - set up DOM event handlers for a new card element {{{ */
  TabletopAlpha.prototype.watch = function ($e) {
    $e
      .on('mouseover', function (event) {
        if ($(event.target).is('.placeholder')) { return; }
        $('.inspect img').attr('src', $(event.target).attr('src'))
                         .css({ visibility: 'visible' });
      })
      .on('mouseout', function (event) {
        $('.inspect img').css({ visibility: 'hidden' });
      })
      .on('dragstart', function (event) {
        var e = $(event.target);
        e.attr({
          dx: event.originalEvent.pageX - e.offset().left,
          dy: event.originalEvent.pageY - e.offset().top,
        });
        event.originalEvent.dataTransfer.setData("id", e.attr('id'));
      });
    return $e;
  };
  /* }}} */
   /* t.mount(element) - set up DOM event handlers {{{ */
  TabletopAlpha.prototype.mount = function ($e) {
    var self = this;
    self.root = $e;

    $e
    .on('dblclick', '.battlefield img.c', function (event) {
      $(event.target).toggleClass('t');
    })
    .on('click', '.library a[rel=draw]', function (event) {
      event.preventDefault();
      $e.find('.draw.strip').template('play-draw', {
        mode: 'draw',
        cards: [self.library.shift()]
      }).toggle();
      $e.find('.wash').toggle();
      self.check();
    })
    .on('click', '.library a[rel=search]', function (event) {
      event.preventDefault();
      self.showSearch(self.library);
    })
    .on('click', '.strip a[rel=put-in-hand]', function (event) {
      var cc = $(event.target).closest('.cards').find('img.c');
      cc.each(function (i, e) {
        $e.find('.hand').append(self.watch($(e)));
      });
    })
    .on('click', '.zone a[rel=search]', function (event) {
      event.preventDefault();
      self.showSearch(self.cardsFrom($(event.target).closest('.zone').find('img.c')));
    })
    .on('click', '.zone a[rel=exile]', function (event) {
      event.preventDefault();
      $e.find('.exile').append($(event.target).closest('.zone').find('img.c'));
      self.check();
    })
    .on('click', '.nap', function (event) {
      self.passTheTurn();
    })
    .on('click', '.strip', function (event) {
      $e.find('.draw.strip').toggle();
      $e.find('.wash').toggle();
    });

    $e.find('.battlefield, .hand, .zone')
    .on('dragover', function (event) {
      /* allow it */
      event.preventDefault();
    });

    $e.find('.battlefield')
    .on('drop', function (event) {
      var field = $(event.target).closest('.battlefield');
      var e = $('[id="'+event.originalEvent.dataTransfer.getData('id')+'"]');

      event.preventDefault();
      var xy = { x: event.originalEvent.pageX - field.offset().left - e.attr('dx'),
                 y: event.originalEvent.pageY - field.offset().top  - e.attr('dy') };
      e.attr({ dx: 0, dy: 0 });
      xy.x = parseInt(xy.x / field.width()  * 100);
      xy.y = parseInt(xy.y / field.height() *  33) * 3;
      // FIXME: xy = self.snap(xy);
      var $c = field.append(e.attr(xy).css({
        top:  xy.y+'%',
        left: xy.x+'%'
      }));
    });

    $e.find('.zone')
    .on('drop', function (event) {
      event.preventDefault();
      var e = $('[id="'+event.originalEvent.dataTransfer.getData('id')+'"]')
                 .attr({ dx: 0, dy: 0 })
                 .removeClass('t');
      $(event.target).closest('.zone')
        .append(e.css({ top: 0, left: 0 }));

      self.check();
    });

    $e.find('.hand')
    .on('drop', function (event) {
      event.preventDefault();
      var e = $('[id="'+event.originalEvent.dataTransfer.getData('id')+'"]')
                 .attr({ dx: 0, dy: 0 })
                 .removeClass('t');
      $(event.target).closest('.hand')
        .append(e.css({ top: 0, left: 0 }));

      self.check();
    });

    this.watch($e.find('.battlefield img.c, .hand img.c, .zones img.c'));

    var open;
    $e.on('click', '.hand', function () {
      if (open) {
        $('#main .hand').animate({ height: '3.733vw' });
      } else {
        $('#main .hand').animate({ height: '12vw' });
      }
      open = !open;
    });

    console.log('registering before unload');
    window.addEventListener('beforeunload', function (event) {
      event.preventDefault();
      event.returnValue = 'Are you sure you want to quit this game?';
      return event.returnValue;
    });

    return this;
  };
  /* }}} */
   /* t.check() - check board state and update as needed{{{ */
  TabletopAlpha.prototype.check = function () {
    this.root.find('.zone').each(function (i, zone) {
      var $zone = $(zone);
      if ($zone.find('img.c').length == 0) {
        $zone.addClass('empty');
      } else {
        $zone.removeClass('empty');
      }
    });
  };
  /* }}} */
   /* t.card(card) - create a DOM element to represent a card {{{ */
  TabletopAlpha.prototype.card = function (c) {
    return $.template('play-card', c);
  };
  /* }}} */
   /* t.cardsFrom($e) - turn a list of IMG.c elements into card objects {{{ */
  TabletopAlpha.prototype.cardsFrom = function ($e) {
    var self = this,
        l = [];

    $e.each(function (i, e) {
      var c = self.cards[$(e).attr('id')];
      if (c) { l.push(c); }
    });
    return l;
  };
  /* }}} */

  TabletopAlpha.prototype.showSearch = function (cards) {
    this.root.find('.wash').toggle();
    this.root.find('.draw.strip').template('play-draw', {
      mode: 'search',
      cards: cards
    }).toggle().on('dragover', function (event) {
      event.preventDefault(); /* allow it */
    });
  };

  TabletopAlpha.prototype.passTheTurn = function () {
    var $ap  = $('.h2h .p.ap');
    var $nap = $('.h2h .p.nap');
    $ap.removeClass('ap').addClass('nap');
    $nap.removeClass('nap').addClass('ap');
  };

  window.TabletopAlpha = TabletopAlpha;
})(window, document);
