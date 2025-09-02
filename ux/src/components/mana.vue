<template>
  <span v-html="colorize(mana)"></span>
</template>

<script>
let manasym = classes => '<i class="ms ms-cost '+classes
                                                   .map(c => 'ms-'+c.toLowerCase())
                                                   .join(' ')+'"></i>';
export default {
  name: 'vcb-mana',
  props: {
    mana: {
      type: String,
      default: '1'
    }
  },
  methods: {
    colorize(s) {
      return s.replace(/^{|\/|}$/g,'').split('}{').filter(x => x != '').map(c => manasym([c])).join('')
    }
  }
}
</script>
