let strftime = function (fmt, d) {
  if (!(d instanceof Date)) {
    let _d = new Date()
    if (!isNaN(d)) {
      _d.setTime(d * 1000); /* epoch s -> ms */
    }
    d = _d
  }
  if (typeof(d) === 'undefined') {
    return ""
  }

  const en_US = {
    pref: {
      /* %c */ datetime: function (d) { return strftime("%a %b %e %H:%M:%S %Y", d); },
      /* %x */ date:     function (d) { return strftime("%m/%d/%Y", d); },
      /* %X */ time:     function (d) { return strftime("%H:%M:%S", d); }
    },
    weekday: {
      abbr: ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'],
      full: ['Sunday',
             'Monday',
             'Tuesday',
             'Wednesday',
             'Thursday',
             'Friday',
             'Saturday']
    },
    month: {
      abbr: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
             'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
      full: ['January',
             'February',
             'March',
             'April',
             'May',
             'June',
             'July',
             'August',
             'September',
             'October',
             'November',
             'December']
    },
    AM: "AM", am: "am", PM: "PM", pm: "pm",
    ordinal: ['th', 'st', 'nd', 'rd', 'th', 'th', 'th', 'th', 'th', 'th', 'th', //  1 - 10
                    'th', 'th', 'th', 'th', 'th', 'th', 'th', 'th', 'th', 'th', // 11 - 20
                    'st', 'nd', 'rd', 'th', 'th', 'th', 'th', 'th', 'th', 'th', // 21 - 30
                    'st'],
    zero:  ['00', '01', '02', '03', '04', '05', '06', '07', '08', '09',
            '10', '11', '12', '13', '14', '15', '16', '17', '18', '19',
            '20', '21', '22', '23', '24', '25', '26', '27', '28', '29',
            '30', '31', '32', '33', '34', '35', '36', '37', '38', '39',
            '40', '41', '42', '43', '44', '45', '46', '47', '48', '49',
            '50', '51', '52', '53', '54', '55', '56', '57', '58', '59'],

    space: [' 0', ' 1', ' 2', ' 3', ' 4', ' 5', ' 6', ' 7', ' 8', ' 9',
            '10', '11', '12', '13', '14', '15', '16', '17', '18', '19',
            '20', '21', '22', '23', '24', '25', '26', '27', '28', '29',
            '30', '31', '32', '33', '34', '35', '36', '37', '38', '39',
            '40', '41', '42', '43', '44', '45', '46', '47', '48', '49',
            '50', '51', '52', '53', '54', '55', '56', '57', '58', '59'],
  }

  let lc = en_US
  let wday

  let inspec = false
  let alt_o  = false

  let s = ''
  for (let i = 0; i < fmt.length; i++) {
    let c = fmt.charCodeAt(i)
    if (inspec) {
      switch (c) {
      // %%   A literal '%' character
      case 37:
        s += '%'
        break

      // %a   The abbreviated name of the day of the week according to the
      //      current locale.
      case 97:
        s += lc.weekday.abbr[d.getDay()]
        break

      // %A   The full name of the day of the week according to the current
      //      locale.
      case 65:
        s += lc.weekday.full[d.getDay()]
        break

      // %b   The abbreviated month name according to the current locale.
      case 98:
        s += lc.month.abbr[d.getMonth()]
        break

      // %h   Equivalent to %b.
      case 104:
        s += lc.month.abbr[d.getMonth()]
        break

      // %B   The full month name according to the current locale.
      case 66:
        s += lc.month.full[d.getMonth()]
        break

      // %c   The preferred date and time representation for the current
      //      locale.
      case 99:
        s += lc.pref.datetime(d)
        break

      // %C   The century number (year/100) as a 2-digit integer
      case 67:
        s += parseInt(d.getFullYear() / 100)
        break

      // %d   The day of the month as a decimal number (range 01 to 31).
      case 100:
        s += lc.zero[d.getDate()]
        break

      // %D   Equivalent to %m/%d/%y.  (Yecch—for Americans only.  Americans
      //      should note that in other countries %d/%m/%y is rather common.
      //      This means that in international context this format is
      //      ambiguous and should not be used.)
      case 68:
        s += strftime("%m/%d/%y", d)
        break

      // %e   Like %d, the day of the month as a decimal number, but a
      //      leading zero is replaced by a space.
      case 101:
        s += d.getDate().toString()+(alt_o ? lc.ordinal[d.getDate()] : '')
        break

      // %E   Modifier: use alternative format, see below.
      case 69:
        // not supported; just skip it
        continue

      // %F   Equivalent to %Y-%m-%d (the ISO 8601 date format).
      case 70:
        s += strftime("%Y-%m-%d", d)
        break

      // %G   The ISO 8601 week-based year (see NOTES) with century as a
      //      decimal number.  The 4-digit year corresponding to the ISO
      //      week number (see %V).  This has the same format and value as
      //      %Y, except that if the ISO week number belongs to the previous
      //      or next year, that year is used instead.
      case 71:
        throw "this strftime() does not support '%G'"

      // %g   Like %G, but without century, that is, with a 2-digit year
      //      (00-99).
      case 103:
        throw "this strftime() does not support '%g'"

      // %H   The hour as a decimal number using a 24-hour clock (range 00 to 23).
      case 72:
        s += lc.zero[d.getHours()]
        break

      // %I   The hour as a decimal number using a 12-hour clock (range 01 to 12)
      case 73:
        s += lc.zero[d.getHours() % 12 == 0 ? 12 : d.getHours() % 12]
        break

      // %j   The day of the year as a decimal number (range 001 to 366).
      case 106:
        throw "this strftime() does not support '%j'"

      // %k   The hour (24-hour clock) as a decimal number (range 0 to 23)
      //      single digits are preceded by a blank.  (See also %H.)
      case 107:
        s += lc.space[d.getHours()]
        break

      // %l   The hour (12-hour clock) as a decimal number (range 1 to 12)
      //      single digits are preceded by a blank.  (See also %I.)
      case 108:
        s += lc.space[d.getHours() % 12 == 0 ? 12 : d.getHours() % 12]
        break

      // %m   The month as a decimal number (range 01 to 12).
      case 109:
        s += lc.zero[d.getMonth()+1]
        break

      // %M   The minute as a decimal number (range 00 to 59).
      case 77:
        s += lc.zero[d.getMinutes()]
        break

      // %n   A newline character.
      case 110:
        s += "\n"
        break

      // %O   Modifier: use alternative format, see below.
      case 79:
        alt_o = true
        continue

      // %p   Either "AM" or "PM" according to the given time value, or the
      //      corresponding strings for the current locale.  Noon is treated
      //      as "PM" and midnight as "AM".
      case 112:
        s += (d.getHours() < 12 ? lc.AM : lc.PM)
        break

      // %P   Like %p but in lowercase: "am" or "pm" or a corresponding
      //      string for the current locale.
      case 80:
        s += (d.getHours() < 12 ? lc.am : lc.pm)
        break

      // %r   The time in a.m. or p.m. notation.  In the POSIX locale this
      //      is equivalent to %I:%M:%S %p.
      case 114:
        s += lc.zero[d.getHours() % 12 == 0 ? 12 : d.getHours() % 12] + ":" +
             lc.zero[d.getMinutes()]                                  + ":" +
             lc.zero[d.getSeconds()]                                  + " " +
             (d.getHours() < 12 ? lc.AM : lc.PM)
        break

      // %R   The time in 24-hour notation (%H:%M).  For a version
      //      including the seconds, see %T below.
      case 82:
        s += lc.zero[d.getHours()] + ":" +
             lc.zero[d.getMinutes()]
        break

      // %s   The number of seconds since the Epoch,
      //      1970-01-01 00:00:00+0000 (UTC).
      case 115:
        s += d.getTime().toString()
        break

      // %S   The second as a decimal number (range 00 to 60).  (The range
      //      is up to 60 to allow for occasional leap seconds.)
      case 83:
        s += lc.zero[d.getSeconds()]
        break

      // %t   A tab character.
      case 116:
        s += "\t"
        break

      // %T   The time in 24-hour notation (%H:%M:%S).
      case 84:
        s += lc.zero[d.getHours()]   + ":" +
             lc.zero[d.getMinutes()] + ":" +
             lc.zero[d.getSeconds()]
        break

      // %u   The day of the week as a decimal, range 1 to 7, Monday being 1.
      //       See also %w.
      case 117:
        wday = d.getDay()
        if (wday == 0) { wday = 7 }
        s += (wday).toString()+(alt_o ? lc.ordinal[wday] : '')
        break

      // %U   The week number of the current year as a decimal number, range
      //      00 to 53, starting with the first Sunday as the first day of
      //      week 01.  See also %V and %W.
      case 85:
        throw "this strftime() does not support '%U'"

      // %V   The ISO 8601 week number (see NOTES) of the current year as a
      //      decimal number, range 01 to 53, where week 1 is the first week
      //      that has at least 4 days in the new year.  See also %U and %W.
      case 86:
        throw "this strftime() does not support '%V'"

      // %w   The day of the week as a decimal, range 0 to 6, Sunday being 0
      //      See also %u.
      case 119:
        s += (d.getDay()).toString()
        break

      // %W   The week number of the current year as a decimal number, range
      //      00 to 53, starting with the first Monday as the first day of
      //      week 01.
      case 87:
        throw "this strftime() does not support '%W'"

      // %x   The preferred date representation for the current locale
      //      without the time.
      case 120:
        s += lc.pref.date(d)
        break

      // %X   The preferred time representation for the current locale
      //      without the date.
      case 88:
        s += lc.pref.time(d)
        break

      // %y   The year as a decimal number without a century (range 00 to 99).
      case 121:
        s += lc.zero[d.getFullYear() % 100]
        break

      // %Y   The year as a decimal number including the century.
      case 89:
        s += d.getFullYear()
        break

      // %z   The +hhmm or -hhmm numeric timezone (that is, the hour and
      //      minute offset from UTC).
      case 122:
        throw "this strftime() does not support '%z'"

      // %Z   The timezone name or abbreviation.
      case 90:
        throw "this strftime() does not support '%Z'"

      default:
        throw "unrecognized strftime sequence '%"+fmt[i]+"'"
      }

      inspec = false
      alt_o  = false
      continue
    }

    if (c == 37) { // %
      inspec = true
      continue
    }

    s += fmt[i]
  }
  return s
}

export default strftime
