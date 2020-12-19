<template>
  <div class="prose">
    <template v-if="transactions.length == 0">
      <h1>Let's Get You A <em>Collection</em>...</h1>
      <p>We don't have a clue what's in your collection.<br>
         Why don't we fix that, and get to know one another better?</p>

      <vcb-transaction-form
        @created-transaction="create_transaction($event)"
        mode="import"
        :transaction="import_txn"></vcb-transaction-form>
    </template>
    <template v-else>
      <h1>Your Magic Collection <em>Through Time!</em></h1>
      <div class="timeline">
        <div class="now summary">
          <h2>Today</h2>
          <ul>
            <li><em>{{ total_cards }}</em> cards (<em>{{ total_unique_cards }}</em> unique)</li>
          </ul>

          <div class="buttons" v-if="mode != 'new'">
            <button class="action" rel="new-buy" @click="start_new_transaction()">Buy / Sell / Trade</button>
          </div>
          <vcb-transaction-form
            v-else
            @created-transaction="create_transaction($event)"
            mode="new"></vcb-transaction-form>
        </div>

        <div v-for="d in monthly_bands" class="aggr" :key="d[0]">
          <h2>{{ heading(d[0]) }} <span :class="gainloss(d[1].total_delta)">{{ d[1].total_delta }} cards <span :class="gainloss(d[1].unique_delta)">({{ d[1].unique_delta }} unique)</span></span></h2>
          <ul>
            <li v-for="txn in d[1].transactions" :class="txn.type" :key="txn.id">
              <em :class="gainloss(txn.total_delta)">{{ txn.total_delta }}</em> cards in {{ txn.dated }} {{ txn.type }} <strong>{{ txn.summary }}</strong>
              <span v-if="txn.paid">($\{{ txn.paid / 100.0 }})</span>
              <span v-for="set in txn.set_gain" :key="set" :class="'ss ss-' + set.toLowerCase()"></span>
              <span v-for="set in txn.set_loss" :key="set" :class="'ss ss-' + set.toLowerCase()"></span>
              <span class="actions"><a href="#" @click="start_edit_transaction(txn.id)">edit</a></span>
              <vcb-transaction-form
                v-if="mode == 'edit' && target == txn.id"
                mode="edit"
                @updated-transaction="update_transaction($event)"
                @deleted-transaction="delete_transaction(txn.id)"
                :transaction="Object.assign({}, txn)"></vcb-transaction-form>
            </li>
          </ul>
        </div>
      </div>
    </template>
  </div>
</template>

<script>
import strftime from '@/lib/strftime'

import VcbTransactionForm from '@/components/transaction-form'

export default {
  name: 'vcb-timeline',
  components: {
    VcbTransactionForm
  },
  props: {
    transactions: {
      type: Array,
      default: () => []
    }
  },
  data: function () {
    return {
      target: '',
      mode: 'view',

      import_txn: {
        summary: 'Collection Import',
        dated:   (new Date()).toISOString().slice(0,10),
        notes:   '',
        disposition: 'buy',
        gain:    '',
        loss:    ''
      }
    }
  },
  computed: {
    total_cards() {
      let v = 0
      this.transactions.forEach(t => {
        v += t.total_card_gain  - t.total_card_loss
      })
      return v
    },
    total_unique_cards() {
      let v = 0
      this.transactions.forEach(t => {
        v += t.unique_card_gain - t.unique_card_loss
      })
      return v
    },

    monthly_bands() {
      let by_date = new Map()

      this.transactions.forEach(t => {
        t.total_delta  = t.total_card_gain  - t.total_card_loss
        t.unique_delta = t.unique_card_gain - t.unique_card_loss

        t.type = ((gained, lost) => {
          if (gained && lost) { return 'trade'; }
          if (gained)         { return 'buy';   }
          if (lost)           { return 'sell';   }
          return 'no-op'
        })(t.total_card_gain > 0, t.total_card_loss > 0)

        var d = t.dated
        d = d ? parseInt(d.replace(/-/g, '')) : undefined
        t._dated = d
        d = d ? (100 * parseInt(d / 100) + 5).toString() : '0'

        if (!by_date.has(d)) {
          by_date.set(d, {
            total_delta:  0,
            unique_delta: 0,
            transactions: []
          })
        }
        let x = by_date.get(d)
        x.transactions.push(t)
        x.total_delta  += t.total_delta
        x.unique_delta += t.unique_delta
        by_date.set(d, x)
      })

      let to_sort = []
      by_date.forEach((band, d) => {
        band.transactions.sort((a, b) => { return b._dated - a._dated; })
        to_sort.push([d, band])
      })

      return to_sort.sort((a, b) => { return b[0] - a[0]; })
    }
  },

  methods: {
    start_new_transaction() {
      event.preventDefault()
      this.mode = 'new'
    },

    start_edit_transaction(id) {
      event.preventDefault()
      this.target = id
      this.mode = 'edit'
    },

    create_transaction(ev) {
      this.transactions.push(ev.transaction)
      this.mode = 'view'
    },

    update_transaction(ev) {
      for (var i = 0; i < this.transactions.length; i++) {
        if (this.transactions[i].id == ev.transaction.id) {
          Object.assign(this.transactions[i], ev.transaction)
          break
        }
      }
      this.mode = 'view'
    },

    delete_transaction(id) {
      for (var i = 0; i < this.transactions.length; i++) {
        if (this.transactions[i].id == id) {
          this.transactions.splice(i, 1)
          break
        }
      }
      this.mode = 'view'
    },

    gainloss(n) {
      return n < 0 ? 'loss' : n > 0 ? 'gain' : ''
    },

    heading(d) {
      return parseInt(d) ? strftime("%B %Y", this.dated(d)) : "The Beginning"
    }
  }
}
</script>

<style lang="scss">
.timeline {
  margin: 1em;
  position:relative;
  font-family: Arial, sans-serif;
  font-size: 1rem;

  &::after {
    background-color: #0a8ab5e3;
    width: 2px;
    height: 100%;
    display: block;
    content: ' ';
    position: absolute;
    top: 20px;
    left: 0px;
  }

  > div {
    margin: 0 0 5em 0;
    position: relative;
    left: 30px;

    &::after {
      border: 4px solid #fff;
      background-color: #166f8c;
      content: ' ';
      height: 12px;
      width: 12px;
      border-radius: 100%;
      display: block;
      position: absolute;
      left: -39px;
      z-index:5;
      top: 2px;
    }
  }

  em {
    font-weight: bold; 
    color: #b44537;
    color: #0071bc;
  }

  .aggr .gain { color: forestgreen; }
  .aggr .loss { color: #b44537; }

  strong {
    font-size: 110%;
    font-weight: bold;
    border-bottom: 2px dotted #888;
  }

  h2 {
    font-size: 1.35rem;
    font-family: Arial, sans-serif;

    > span {
      font-size: 60%;
    }
  }

  ul {
    display: flex;
    flex-flow: column wrap;
    padding-left: 1em;
    line-height: 1.6em;
    margin: 1em 0;
  }

  .summary ul {
    line-height: 1em;
    margin: 2em 0;

    li {
      font-size: 140%;
      line-height: 1.3em;
    }
  }
  .aggr li {
    &.buy, &.sell {
      > em:first-child {
        width: 5.5ex;
        display: inline-block;
        text-align: right;
      }
      > em:nth-child(2) {
        width: 2.2ex;
        display: inline-block;
        text-align: right;
      }
    }
  }
  .buttons {
    display: flex;
    flex-flow: row nowrap;
    margin: 0 1em;
  }
  a {
    color: #0071BC;
    text-decoration: none;
  }
  .actions {
    font-size: 80%;
    visibility: hidden;
    display: inline-block;
    padding-left: 0.5em;
  }
  > div:hover .actions { visibility: visible; }
}
</style>
