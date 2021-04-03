<template>
  <div>
    <div class="modal-bg"></div>
    <div class="modal-fg card-detail" v-if="card">
      <div>
        <div>
          <vcb-card :card="card"></vcb-card>
        </div>
        <div class="detail">
          <a class="close" href="#" @click.prevent="$emit('close')">X</a>
          <ul class="info">
            <li><strong>{{ card.name }}</strong> <a rel="debug" href="#" @click.prevent="debug = !debug">debug</a></li>
            <li><em>{{ card.type }}</em></li>
            <li>{{ card.set.name }} ({{ card.set.code }})</li>
            <li>#{{ card.number }}/{{ card.set.total }}</li>
            <li><vcb-mana :mana="card.color"></vcb-mana></li>
            <li><strong>CMC:</strong> {{ card.cmc }}</li>
            <li>{{ card.rarity }}</li>
          </ul>
          <ul class="owned">
            <li><strong>Owned:</strong> {{ card.owned }}</li>
          </ul>
          <ul>
            <li class="oracle"><p v-for="(line, i) in oracles(card)" :key="i" v-html="line"></p></li>
          </ul>
          <div class="debug" v-if="debug">
            <pre><code>{{ dump(card) }}</code></pre>
          </div>
          <p v-if="card.price" class="price">
            {{ price(card.price) }}
            <span v-if="card.price > 999" class="w1">remember kids, M:tG is <strong>NOT</strong> an investment...</span>
            <span v-else-if="card.price > 99" class="w2">oof</span>
            <span v-else-if="card.price > 9" class="w3">that's a bit much, don't you think?</span>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'

import VcbCard from '@/components/card'
import VcbMana from '@/components/mana'

export default {
  name: 'vcb-card-detail',
  props: ['card'],
  components: {
    VcbCard,
    VcbMana
  },
  data() {
    return {
      debug: false,
    }
  },
  computed: {
    ...mapGetters(['imgroot']),
  },
  methods: {
    price(d) {
      if (typeof(d) === 'undefined' || d == '') {
        return ''
      }
      return '$'+parseFloat(d).toFixed(2).split('').reverse().join('').match(/.{1,3}/g).join(',').split('').reverse().join('').replace(/,\./, '.')
    },
    oracles(card) {
      return this.symbolize(card.oracle.replace('//', "\n<hr>\n")).split(/\n+/)
    },
    dump(card) {
      return JSON.stringify(card, null, '  ')
    }
  }
}
</script>

<style lang="scss" scoped>
.modal-fg.card-detail > div {
  position: relative;
  background-color: #fff;
  width: 70vw;
  min-height: 70vh;
  margin: 1em auto;
  border-radius: 18px;
  padding: 18px;
  box-sizing: border-box;

  font-family: sans-serif;
  display: flex;
  flex-direction: row;

  .card {
    transform: rotate(5deg) translate(-100px, 0);

    .face {
    display: none;
      width: 50.31vh;
      height: 70vh;
      border-radius: 2.4vh;
      overflow: hidden;

      span {
        border-radius: none;
      }
    }
  }

  .detail {
    font-size: 11.5pt;

    display: flex;
    flex-direction: column;
    justify-content: flex-start;

    h1 {
      font-size: 180%;
      font-weight: 600;
    }

    em     { font-style:  italic; }
    strong { font-weight: bold; }
    ul {
      margin: 0 0 1em 0;

      li p {
        margin: 8pt 0;
      }
    }

    .where {
      flex-grow: 1;
      flex-basis: auto;
    }

    .tags {
      display: flex;
      flex-flow: row wrap;

      li {
        background-color: #ba1fba;
        border-radius: 4px;
        padding: 4px 8px;
        color: #fff;
        font-size: 90%;
        font-family: monospace;
        font-weight: bold;
        margin: 2px;
      }
    }

    a[rel="debug"] {
      color: transparent;
      &:hover {
        color: #ccc;
      }
      outline: none;
      font-size: 11px;
      text-decoration: none;
    }

    .debug {
      pre {
        font-family: monospace;
        display: block;
        height: 14em;
        overflow: auto;
        border: 4px solid #86ae88;
        background-color: #2d3e34;
        color: #d1eed7;
        font-size: 12px;
        line-height: 15px;
        border-width: 5px 0;
        padding: 1em;
        white-space: pre-line;
      }
    }

    .price {
      font-size: 48pt;
      align-self: flex-end;
      font-family: sans-serif;
      font-weight: bold;
      color: #888;

      .w1, .w2, .w3 {
        font-style: italic;
        font-size: 9pt;
        display: block;
        text-align: right;
        opacity: 0.7;
      }
      .w1 { color: darkred; }
      .w2 { color: #ba6c2a; }
      .w3 { color: inherit; }
    }
  }
}
</style>
