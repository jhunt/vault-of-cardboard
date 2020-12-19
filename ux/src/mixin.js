import strftime from "@/lib/strftime"

let manasym = classes => '<i class="ms ms-cost '+classes
                                                   .map(c => 'ms-'+c.toLowerCase())
                                                   .join(' ')+'"></i>';


export default {
  methods: {
    strftime: strftime,

    dated(yyyymmdd, unknown) {
      yyyymmdd = (yyyymmdd || '').toString().replace(/-/g, '').replace(/T\d\d:\d\d:\d\d.\d*Z$/, '')
      if (yyyymmdd.length == 4) { yyyymmdd += '0101'; }
      if (yyyymmdd.length == 6) { yyyymmdd +=   '01'; }
      if (yyyymmdd.length != 8) { return unknown; }

      var dd   =  yyyymmdd                        % 100
      var mm   = (yyyymmdd -          dd) /   100 % 100
      var yyyy = (yyyymmdd - mm*100 - dd) / 10000
      return new Date(yyyy, mm - 1, dd)
    },

    dollar$(n) {
      let value = parseInt(n * 100)
      if (isNaN(value)) { return ''; }

      let cents = value % 100
      let dollars = parseInt(value / 100)

      dollars = dollars.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")
      if (cents < 10) {
        return dollars + ".0" + cents.toString()
      } else {
        return dollars + "." + cents.toString()
      }
    },

    symbolize(s) {
      return s.replace(/{(.+?)}/g, (_m, found) =>
        manasym(
          found
            .toLowerCase().split('/')
            .map(sym => sym == 't' ? 'tap' : sym == 'q' ? 'untap' : sym)))
    },

    goto(where) {
      let  here = this.$route.fullPath
      let there = this.$router.resolve(where).route.fullPath
      if (here != there) {
        this.$router.push(where)
      }
    }
  }
}
