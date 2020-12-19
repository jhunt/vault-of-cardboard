<template>
  <form :class="'deck data-entry ' + mode" @submit.prevent="save()">
    <h2 v-if="mode == 'new'">Craft a New Deck</h2>
    <h2 v-else-if="mode == 'edit'">Update &ldquo;{{ deck.title }}&rdquo;</h2>
    <div class="band">
      <div>
        <div class="control">
          <label for="code">Internal Identifier</label>
           <input type="text" name="code" v-model="deck.code"
                  class="autofocus"
                  placeholder="A unique ID to use with 'in:...' queries"
                  data-validate="present;min=2;max=40"
                  data-error-if-missing="Please provide an internal identifier, for use in querying the contents of this deck."
                  data-error-out-of-bounds="Identifiers must be between 2 and 40 characters long."
                  autocomplete="off">

        </div>
      </div>
    </div>

    <div class="band">
      <div>
        <div class="control">
          <label for="title">Title of Deck</label>
          <input type="text" name="title" v-model="deck.title"
                 class="autofocus"
                 placeholder="A name for this deck, which will be displayed to others."
                 data-validate="present;min=2"
                 data-error-if-missing="Please provide a name for this deck."
                 data-error-out-of-bounds="Deck names must be at least two characters long."
                 autocomplete="off">
        </div>
      </div>
    </div>

    <div class="band">
      <div>
        <div class="control">
          <label for="description">Description / Notes</label>
          <textarea name="description" v-model="deck.description"
                    placeholder="(feel free to expound on the premise and power of your deck...)"
                    data-validate="max=2000"></textarea>
        </div>
      </div>
    </div>

    <div class="band">
      <div>
        <div class="control">
          <label for="main">Maindeck</label>
          <textarea name="main" v-model="deck.main"
                    @blur="validate()"
                    class="cdif"
                    data-validate="present;max=100k"></textarea>
        </div>
        <vcb-gainloss-status :status="status"></vcb-gainloss-status>
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
    <vcb-clarifier v-for="problem in problems" :key="problem.id"
                   @clarified="clarified($event.problem, $event.replacements)"
                   :problem="problem"></vcb-clarifier>

    <div v-if="problems.length == 0">
      <button class="default safe" type="submit">{{ mode == 'new' ? "Create" : "Update" }} Deck</button>
      <button v-if="mode == 'edit'" @click.prevent="remove()" class="danger">Delete This Deck</button>
    </div>
  </form>
</template>

<script>
import { mapGetters } from 'vuex'
import cardboard from '@/lib/cardboard/index'

import VcbClarifier from '@/components/clarifier'
import VcbGainlossStatus from '@/components/gainloss-status'

export default {
  name: 'vcb-deck-form',
  components: {
    VcbClarifier,
    VcbGainlossStatus
  },
  props: {
    mode: String,
    status: Function,
    deck: {
      type: Object,
      default: function() {
        return {
          title:       '',
          code:        '',
          description: '',
          main:        '',
          side:        '',
          maybe:       ''
        }
      }
    }
  },
  data() {
    return {
      problems: []
    }
  },
  computed: {
    ...mapGetters(['session', 'vault'])
  },
  methods: {
    save() {
      if (this.deck.id) {
        cardboard.API.patch_deck(this.session, this.deck)
          .then(data => this.$emit('updated-deck', data))

      } else {
        cardboard.API.post_deck(this.session, this.deck)
          .then(data => this.$emit('created-deck', data))
      }
    },
    remove() {
      cardboard.API.delete_deck(this.session, this.deck.id)
        .then(() => this.$emit('deleted-deck', this.deck.id))
    },

    validate() {
      this.problems = cardboard.CDIF.validate(this.deck.main, this.vault, 1)
    },

    clarified(problem, replacements) {
      let lines = this.deck.main.split("\n")
      lines.splice(problem.line - 1, 1, ...replacements.map(r => r[0]+'x '+r[1]))
      this.deck.main = lines.join("\n")

      this.validate()
    }
  }
}
</script>
