<template>
  <form :class="'transaction data-entry ' + mode" @submit="save()">
    <h2 v-if="mode == 'new'">Buy / Sell / Trade Cards</h2>
    <h2 v-else-if="mode == 'edit'">Update Buy / Sell / Trade Details</h2>
    <template v-if="mode == 'import'">
      <input type="hidden" name="summary" v-model="transaction.summary">
      <input type="hidden" name="dated" v-model="transaction.dated">
    </template>
    <template v-else>
      <div class="band">
        <div>
          <div class="control">
            <label for="summary">Summary / Transaction Name</label>
            <input type="text" name="summary" v-model="transaction.summary"
                   class="autofocus"
                   placeholder="A (unique-ish) name for this transaction"
                   data-validate="present;min=3;max=50"
                   data-error-if-missing="Please provide a name, which will be displayed in the timeline."
                   data-error-out-of-bounds="Names must be between 3 and 50 characters long."
                   autocomplete="off">
          </div>
        </div>
      </div>
      <div class="band">
        <div>
          <div class="control">
            <label for="dated">Date of Transaction</label>
            <input type="date" name="dated" v-model="transaction.dated"
                   data-validate="present;format=date;happened"
                   autocomplete="off">
          </div>
        </div>
      </div>
      <div class="band">
        <div>
          <div class="control">
            <label for="notes">Notes</label>
            <textarea name="notes" v-model="transaction.notes"
                      placeholder="(notes, thoughts, context - only visible to you...)"
                      data-validate="max=2000"></textarea>
          </div>
        </div>
      </div>
    </template>
    <div class="band" v-if="mode != 'import'">
      <div>
        <div class="control">
          <label>Type of Transaction</label>

          <ul><li>
            <input type="radio" name="disposition" value="buy" v-model="transaction.disposition">
            <label><strong>Buy</strong> &mdash; this transaction <em>adds</em> cards to the collection.</label>
          </li><li>
            <input type="radio" name="disposition" value="sell" v-model="transaction.disposition">
            <label><strong>Sell</strong> &mdash; this transaction <em>removes</em> cards from the collection.</label>
          </li><li>
            <input type="radio" name="disposition" value="trade" v-model="transaction.disposition">
            <label><strong>Trade</strong> &mdash; this transaction does <em>both</em>.</label>
          </li></ul>
        </div>
      </div>
    </div>
    <div class="band" v-if="transaction.disposition == 'buy'">
      <div v-if="mode != 'import'">
        <div class="control">
          <label>Amount Spent (optional)</label>
          <input type="text" :value="getPaid()"  @change="setPaid($event.target.value)"
                 placeholder="(how much did you spend on these new cards?)">
        </div>
      </div>
    </div>
    <div class="band" v-if="transaction.disposition == 'buy'">
      <div v-if="mode != 'import'"
           class="gainloss">
        <div class="control gained">
          <label for="gain">Gained Cards</label>
          <textarea name="gain" v-model="transaction.gain"
                    class="cdif"
                    @blur="validate('gain')"
                    data-validate="present;max=100k"></textarea>
        </div>
        <vcb-gainloss-status :status="status()"></vcb-gainloss-status>
      </div>
      <div v-else
           class="gainloss">
        <div class="control import gained">
          <label for="gain">What cards do you have?</label>
          <textarea name="gain" v-model="transaction.gain"
                    class="cdif"
                    @blur="validate('gain')"
                    data-validate="present;max=100k"></textarea>
        </div>
        <vcb-gainloss-status :status="status()"></vcb-gainloss-status>
      </div>
      <div>
        <div class="help no-frills">
          <p>VIF format is designed to be simple.<br>
             First, specify how many cards are gained or lost:</p>
          <p class="example"><span class="segment highlight">4x</span> <span class="segment">MIR</span> <span class="segment">Lion's Eye Diamond</span></p>
          <p>Then, what set the printed cards are from:</p>
          <p class="example"><span class="segment">4x</span> <span class="segment highlight">MIR</span> <span class="segment">Lion's Eye Diamond</span></p>
          <p>Then the full card oracle name:</p>
          <p class="example"><span class="segment">4x</span> <span class="segment">MIR</span> <span class="segment highlight">Lion's Eye Diamond</span></p>
        </div>
      </div>
    </div>
    <template v-if="transaction.disposition == 'buy'">
      <vcb-clarifier v-for="problem in problems.gain" :key="problem.id"
                     @clarified="clarified('gain', $event.problem, $event.replacements)"
                     :problem="problem"></vcb-clarifier>
    </template>

    <div class="band" v-else-if="transaction.disposition == 'sell'">
      <div class="gainloss">
        <div class="control lost">
          <label for="vif">Cards Sold &mdash; these will be removed from your collection.</label>
          <textarea name="loss" v-model="transaction.loss"
                    class="cdif"
                    @blur="validate('loss')"
                    data-validate="present;max=100k"></textarea>
          <vcb-gainloss-status :status="status()"></vcb-gainloss-status>
        </div>
        <vcb-clarifier v-for="problem in problems.loss" :key="problem.id"
                       @clarified="clarified('loss', $event.problem, $event.replacements)"
                       :problem="problem"></vcb-clarifier>
      </div>
      <div>
        <div class="help no-frills">
          <p>VIF format is designed to be simple.<br>
             First, specify how many cards are gained or lost:</p>
          <p class="example"><span class="segment highlight">4x</span> <span class="segment">MIR</span> <span class="segment">Lion's Eye Diamond</span></p>
          <p>Then, what set the printed cards are from:</p>
          <p class="example"><span class="segment">4x</span> <span class="segment highlight">MIR</span> <span class="segment">Lion's Eye Diamond</span></p>
          <p>Then the full card oracle name:</p>
          <p class="example"><span class="segment">4x</span> <span class="segment">MIR</span> <span class="segment highlight">Lion's Eye Diamond</span></p>
        </div>
      </div>
    </div>

    <template v-else-if="transaction.disposition == 'trade'">
      <div class="band">
        <div class="gainloss">
          <div class="control gained">
            <label for="vif">Cards Gained In Trade&mdash; these will be added to your collection.</label>
            <textarea name="gain" v-model="transaction.gain"
                      class="cdif"
                      @blur="validate('gain')"
                      data-validate="present;max=100k"></textarea>
          </div>
          <vcb-gainloss-status :status="status()"></vcb-gainloss-status>
          <vcb-clarifier v-for="problem in problems.loss" :key="problem.id"
                         @clarified="clarified('loss', $event.problem, $event.replacements)"
                         :problem="problem"></vcb-clarifier>
        </div>
        <div>
          <div class="help no-frills">
            <p>VIF format is designed to be simple.<br>
               First, specify how many cards are gained or lost:</p>
            <p class="example"><span class="segment highlight">4x</span> <span class="segment">MIR</span> <span class="segment">Lion's Eye Diamond</span></p>
            <p>Then, what set the printed cards are from:</p>
            <p class="example"><span class="segment">4x</span> <span class="segment highlight">MIR</span> <span class="segment">Lion's Eye Diamond</span></p>
            <p>Then the full card oracle name:</p>
            <p class="example"><span class="segment">4x</span> <span class="segment">MIR</span> <span class="segment highlight">Lion's Eye Diamond</span></p>
          </div>
        </div>
      </div>

      <div class="band">
        <div class="gainloss">
          <div class="control lost">
            <label for="vif">Cards Given In Trade &mdash; these will be removed from your collection.</label>
            <textarea name="loss" v-model="transaction.loss"
                      class="cdif"
                      @blur="validate('loss')"
                      data-validate="present;max=100k"></textarea>
          </div>
        </div>
        <div>
          <div class="help no-frills">
            <p>VIF format is designed to be simple.<br>
               First, specify how many cards are gained or lost:</p>
            <p class="example"><span class="segment highlight">4x</span> <span class="segment">MIR</span> <span class="segment">Lion's Eye Diamond</span></p>
            <p>Then, what set the printed cards are from:</p>
            <p class="example"><span class="segment">4x</span> <span class="segment highlight">MIR</span> <span class="segment">Lion's Eye Diamond</span></p>
            <p>Then the full card oracle name:</p>
            <p class="example"><span class="segment">4x</span> <span class="segment">MIR</span> <span class="segment highlight">Lion's Eye Diamond</span></p>
          </div>
        </div>
      </div>
    </template>

    <template>
      <div v-if="ok">
        <button class="default safe" type="submit">{{ mode == 'import' ? "Import" : "Update" }} Collection</button>
        <button v-if="mode == 'edit'" @click="remove()" class="danger">Delete This Transaction</button>
      </div>
    </template>
  </form>
</template>

<script>
import { mapGetters } from 'vuex'
import cardboard from '@/lib/cardboard/index'

import VcbClarifier      from '@/components/clarifier'
import VcbGainlossStatus from '@/components/gainloss-status'

export default {
  name: 'vcb-transaction-form',
  components: {
    VcbClarifier,
    VcbGainlossStatus
  },
  props: {
    mode: String,
    transaction: {
      type: Object,
      default: function() {
        return {
          summary:     '',
          dated: (new Date()).toISOString().slice(0,10),
          notes:       '',
          gain:        '',
          loss:        '',
          disposition: 'buy',
        }
      }
    }
  },
  data() {
    return {
      ok: true,
      working: false,
      problems: {
        gain: [],
        loss: []
      },
      valid: {
        gain: "",
        loss: ""
      }
    }
  },
  computed: {
    ...mapGetters(['vault', 'session'])
  },
  methods: {
    today() {
      // YYYY-MM-DD format = the first 10 chars in ISO format
      return (new Date()).toISOString().slice(0,10)
    },

    validate(bucket) {
      this.working = true

      window.setTimeout(() => { // add a barely perceptible delay
        if (bucket == 'gain') {
          this.problems.gain = cardboard.CDIF.validate(this.transaction.gain, this.vault, 1)
          this.valid.gain = this.transaction.gain
        } else {
          this.problems.loss = cardboard.CDIF.validate(this.transaction.loss, this.vault, 1)
          this.valid.loss = this.transaction.loss
        }
        this.ok = this.can_we_submit()
        this.working = false
      }, 400)
    },

    changed() {
      let trim = s => s.replace(/(^\s+|\s+$)/g, '').replace(/\s+/g, ' ')
      return trim(this.valid.gain) != trim(this.transaction.gain)
          || trim(this.valid.loss) != trim(this.transaction.loss)
    },

    setPaid(v) {
      v = v.replace(/^\s*$\s*/, '')
      this.transaction.paid = parseInt(parseFloat(v) * 100)
      if (isNaN(this.transaction.paid)) {
        this.transaction.paid = null
      }
    },

    getPaid() {
      if (!this.transaction.paid) {
        return ""
      }
      let dollars = parseInt(this.transaction.paid / 100)
      let cents = this.transaction.paid % 100
      if (cents < 10) {
        return dollars.toString() + '.0' + cents.toString()
      } else {
        return dollars.toString() + '.' + cents.toString()
      }
    },

    are_we_ok() {
      if (this.changed()) {
        return false
      }
      if (this.transaction.disposition == 'buy') {
        return this.transaction.gain.replace(/\s+/, '') != "" && this.can_we_submit()
      }
      if (this.transaction.disposition == 'sell') {
        return this.transaction.loss.replace(/\s+/, '') != "" && this.can_we_submit()
      }
      if (this.transaction.disposition == 'trade') {
        return this.transaction.gain.replace(/\s+/, '') != ""
            && this.transaction.loss.replace(/\s+/, '') != ""
            && this.can_we_submit()
      }
    },

    status() {
      if (this.working) {
        return "working"
      }
      if (this.are_we_ok()) {
        return "ok"
      }
      return "invalid"
    },

    can_we_submit() {
      if (this.transaction.disposition == 'buy') {
        return this.problems.gain.length == 0
      }
      if (this.transaction.disposition == 'sell') {
        return this.problems.loss.length == 0
      }
      if (this.transaction.disposition == 'trade') {
        return this.problems.gain +
               this.problems.loss == 0
      }
    },

    save() {
      event.preventDefault()

      if (this.transaction.id) {
        cardboard.API.patch_transaction(this.session, this.transaction)
          .then(data => {
            this.$store.dispatch('reloadCollection', this.session)
            this.$emit('updated-transaction', data)
          })

      } else {
        cardboard.API.post_transaction(this.session, this.transaction)
          .then(data => {
            this.$store.dispatch('reloadCollection', this.session)
            this.$emit('created-transaction', data)
          })
      }
    },
    remove() {
      event.preventDefault()

      cardboard.API.delete_transaction(this.session, this.transaction.id)
        .then(() => this.$emit('deleted-transaction', this.transaction.id))
    },

    clarified(type, problem, replacements) {
      this.working = true
      if (type == 'gain') {
        this.problems.gain = []
      } else {
        this.problems.loss = []
      }

      let lines = this.transaction[type].split("\n")
      lines.splice(problem.line - 1, 1, ...replacements.map(r => r[0]+'x '+r[1]))
      this.transaction[type] = lines.join("\n")

      this.validate(type)
    }
  }
}
</script>
