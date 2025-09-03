<template>
  <div class="card-detail-bgfill modal">
    <div class="card-detail">
      <div>
        <vcb-card :card="card"></vcb-card>
      </div>
      <div class="detail">
        <div class="card-color-identity" :style="`background: ${colorband(card.color)}`"></div>
        <a rel="close" @click.prevent="$emit('close')">✕</a>
        <a rel="debug" @click.prevent="debug = !debug">debug</a>
        <ul class="info block-1">
          <li class="card-name">{{ card.name }}</li>
          <li class="card-mana-cost"><vcb-mana :mana="card.cost"></vcb-mana></li>
          <li class="card-type">{{ card.type }}</li>
          <li class="card-cmc"><strong>CMC:</strong> {{ card.cmc }}</li>

          <li class="oracle" v-if="card.oracle != ''"><p v-for="(line, i) in oracles(card)" :key="i" v-html="line"></p></li>
          <li class="flavor" v-if="card.flavor != ''"><p v-for="(line, i) in flavors(card)" :key="i" v-html="line" :class="line.match(/^—/) ? 'cite' : ''"></p></li>
          <li class="card-set">
            <span :class="'ss ss-' + card.set.code.toLowerCase()"></span>
            <span class="card-set-info-1">
              <span class="card-set-code">{{ card.set.code }}</span>
              <span :class="'card-set-rarity ' + card.rarity">{{ card.rarity }}</span>
              <span class="card-set-number">{{ card.number }}</span> / <span class="card-set-total">{{ card.set.total }}</span>
            </span>
            <span class="card-set-info-2">
              <span class="card-set-name">{{ card.set.name }}</span>
              <span class="card-set-release-date">{{ card.set.release.toString().replace(/^(\d{4})(\d\d)(\d\d)$/, '$1/$2/$3') }}</span>
            </span>
            <span class="card-set-info-3">
              Illustrated by <span class="card-artist">{{ card.artist }}</span>
            </span>
            <span v-if="card.price" class="card-price">
              <span class="price-usd">{{ price(card.price) }}</span>
              <span v-if="card.price > 999" class="price-warning w1">remember kids, M:tG is <strong>NOT</strong> an investment...</span>
              <span v-else-if="card.price > 99" class="price-warning w2">oof</span>
              <span v-else-if="card.price > 9" class="price-warning w3">that's a bit much, don't you think?</span>
            </span>
          </li>
          <li class="card-owned">
            <span v-if="!session"><router-link to="/signin">Sign in</router-link> to see if this card is in your collection.</span>
            <span v-else-if="card.owned > 0">You own {{ card.owned }} of this card.</span>
            <span v-else>You do not own this card.</span>
          </li>
          <li class="card-legality">
            <span v-for="l in legalities" :key="l[0]" :class="[card.flags.indexOf(l[0]) >= 0 ? 'is' : 'not', 'legal-in', l[2]].join(' ')">{{ l[1] }}</span>
          </li>
        </ul>
        <div class="debug" v-if="debug">
          <pre><code>{{ dump(card) }}</code></pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import VcbCard from '@/components/card'
import VcbMana from '@/components/mana'
import { mapGetters } from 'vuex'

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
    legalities() {
      return [
        ['B', 'Brawl', 'brawl'],
        ['E', 'Commander', 'edh'],
        ['d', 'Duel', 'duel'],
        ['j', 'Frontier', 'frontier'],
        ['f', 'Future', 'future'],
        ['h', 'Historic', 'historic'],
        ['l', 'Legacy', 'legacy'],
        ['m', 'Modern', 'modern'],
        ['o', 'Old School', 'old-school'],
        ['P', 'Pauper', 'pauper'],
        ['$', 'Penny', 'penny'],
        ['p', 'Pioneer', 'pioneer'],
        ['s', 'Standard', 'standard'],
        ['v', 'Vintage', 'vintage'],
      ];
    },
    ...mapGetters(['session'])
  },
  methods: {
    colorband(c) {
      const rgba = {
        'W': 'rgb(238, 230, 181)',
        'U': 'rgb(55, 95, 182)',
        'B': 'rgb(51, 51, 51)',
        'R': 'rgb(174, 68, 65)',
        'G': 'rgb(47, 122, 91)',
        '':  'rgb(203, 201, 198)',
      };
      const colors = c == '' ? [''] : c.split('');
      let css = 'linear-gradient(90deg';
      colors.forEach((x,i) => css += ', ' + rgba[x] + ' ' + (i * (100 / colors.length)) + '% ' + ((i + 1) * (100 / colors.length)) + '%');
      css += ')';
      return css;
    },
    price(d) {
      if (typeof(d) === 'undefined' || d == '') {
        return ''
      }
      return '$'+parseFloat(d).toFixed(2).split('').reverse().join('').match(/.{1,3}/g).join(',').split('').reverse().join('').replace(/,\./, '.')
    },
    oracles(card) {
      return this.symbolize(card.oracle.replace('//', "\n<hr>\n")).split(/\n+/)
    },
    flavors(card) {
      return this.symbolize(card.flavor).split(/\n+/)
    },
    dump(card) {
      return JSON.stringify(card, null, '  ')
    }
  }
}
</script>

<style lang="scss" scoped>
.card-detail-bgfill {
  position: fixed;
  z-index: 10;
  background-color: rgba(255, 255, 255, 0.9);
  top: 0; right: 0; bottom: 0; left: 0;

  .card-detail {
    max-width: 1200px;
    margin: 4em auto 1em auto;
    padding: 0 2em;

    overflow: visible;
    border-radius: 1em;
    color: #fff;

    display: grid;
    grid-template-columns: 2fr 3fr;

    @media screen and (max-width: 768px) {
      grid-template-columns: 0 1fr;
    }

    font-family: sans-serif;

    > div:first-child {
      z-index: 5;
    }

    .card .face {
      clip-path: url(#card-clip);
    }

    .detail {
      position: relative;
      max-height: calc(100vh - 5em - 2em);
      overflow-y: auto;
      scrollbar-color: #555 #222;
      scrollbar-width: thin;

      background-image: url(/img/bgtexture.jpg);
      background-repeat: no-repeat;
      background-size: cover;
      background-color: #333;
      border-radius: 1em;
      padding: 1em 1em 1em 2.5em;

      a[href] {
        text-decoration: none;
        border-bottom: 1px dotted;
        color: #a6c1dd;
      }

      a[rel] {
        text-decoration: none;
        border: none;
        font-family: sans-serif;
        color: white;
        cursor: pointer;
      }
      a[rel=close] {
        position: absolute;
        top: 16px; right: 16px;
      }
      a[rel=debug] {
        position: absolute;
        top: 16px;
        left: 40px;
        font-size: 8pt;
        opacity: 0;

        &:hover { opacity: 1; }
      }

      .debug pre {
        display: block;
        padding: 1em;
        overflow-x: auto;
        max-width: calc(100% - 2em);
        background-color: #000;
        border-radius: 5pt;

        code {
          color: #ccc;
          font-size: 9pt;
          font-family: monospace;
          white-space: pre-wrap;
        }
      }

      .card-color-identity {
        height: 8px;
        position: absolute;
        left: 0; top: 0; right: 0;
      }

      .info {
        display: grid;
        grid-template-columns: repeat(16, 1fr);

        li {
          grid-column: 1 / 17;
          margin-bottom: 16pt;

          &.card-name, &.card-mana-cost {
            grid-row: 1 / 2;
            margin-bottom: 0;
            margin-top: 18pt;
          }
          &.card-name {
            font-size: 18pt;
          }
          &.card-mana-cost {
            text-align: right;
          }

          &.card-type, &.card-cmc {
            grid-row: 2 / 3;
            line-height: 2em;
          }
          &.card-cmc {
            font-size: 9pt;
            text-align: right;
          }

          &.oracle {
            border-top: 1px solid #666;
            padding: 16pt 8pt;
            border-bottom: 1px solid #666;
            font-size: 14pt;
            line-height: 1.3em;

            p+p {
              margin: 1em 0 0 0;
            }
          }

          &.flavor {
            border-bottom: 1px solid #666;
            padding: 0 8pt 16pt 8pt;
            font-style: italic;
            font-size: 12pt;
            line-height: 1.3em;

            p.cite { text-align: right; }

            p+p {
              margin: 1em 0 0 0;
            }
          }

          &.card-set {
            display: grid;
            grid-template-columns: 32px 1fr;

            .ss {
              grid-row: 1 / 3;
              font-size: 28px;
            }
            .card-set-info-1 {
              display: flex;
              flex-direction: row;
              font-family: monospace;
              font-size: 8pt;

              .card-set-code {
                margin: 0 1ex 0 0;
                padding: 2pt 4pt;
                border-radius: 3pt;
                border: 1px solid;
                font-size: 6pt;
                position: relative;
                left: -1px;
              }
              .card-set-rarity {
                margin: 0 1ex 0 0;
                padding: 2pt 4pt;
                text-transform: capitalize;

                &.common   { }
                &.uncommon { color: silver; }
                &.rare     { color: gold;   }
                &.mythic   { color: orange; }
              }
              .card-set-number, .card-set-total {
                font-size: 11pt;
              }
            }
            .card-set-info-2 {
              grid-column: 2 / 3;
              font-size: 12pt;
              line-height: 1.6em;

              .card-set-name {
                margin: 0 1ex 0 0;
              }
            }
            .card-set-info-3 {
              grid-column: 2 / 3;
              font-size: 12pt;
              line-height: 1.6em;
            }

            .card-price {
              grid-column: 3 / 4;
              grid-row: 1 / 5;
              text-align: right;

              .price-usd {
                font-size: 18pt;
              }
              .price-warning {
                display: block;
                font-size: 10pt;
                max-width: 8em;
                font-style: italic;
                margin-top: 1ex;

                &.w1 { color: #ff7676; }
                &.w2 { color: #ef8f3e; }
                &.w3 { color: #bcbcbc; }
              }
            }
          }

          &.card-legality {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            @media screen and (max-width: 768px) {
              grid-template-columns: repeat(2, 1fr);
            }
            grid-gap: 6px 1em;

            span.is::before,
            span.not::before {
              font-size: 12pt;
              width: 16pt;
              display: inline-block;
              text-align: center;
            }
            span.is::before {
              content: '✔';
              color: #10ce10;
            }
            span.not::before {
              content: '✘';
              color: #636363;
            }
          }
        }
      }
    }
  }
}
</style>
