<template>
  <div class="decks listing prose">
    <h1>Your Magic: the Gathering Decks</h1>
    <div class="buttons" v-if="mode == 'view'">
      <button class="action" @click.prevent="start_new_deck()">Craft New Deck</button>
    </div>
    <vcb-deck-form
      v-else
      @created-deck="create_deck($event)"
      mode="new"></vcb-deck-form>

    <table class="decks sortable listing">
      <thead><tr>
        <th class="sortable">Code</th>
        <th class="sortable">Name</th>
        <th class="sortable" data-sort-as="number">Created</th>
        <th class="sortable" data-sort-as="number">Last Updated</th>
      </tr></thead>
      <tbody>
        <tr v-for="deck in decks" :key="deck.id">
          <td><router-link :to='{name: "deck", params: { uid: deck.collector, did: deck.id }}'>{{ deck.code }}</router-link></td>
          <td><router-link :to='{name: "deck", params: { uid: deck.collector, did: deck.id }}'>{{ deck.title }}</router-link></td>
          <td :data-sort="deck.created_at">{{ strftime("%B %Oe, %Y", dated(deck.created_at)) }}</td>
          <td :data-sort="deck.updated_at">{{ strftime("%B %Oe, %Y", dated(deck.updated_at)) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import VcbDeckForm from '@/components/deck-form'

export default {
  name: 'vcb-deck-list',
  components: {VcbDeckForm},
  props: ['decks'],
  data: function () {
    return {
      mode: 'view',
      deck: null
    }
  },
  mounted() {
    //$('#main table.sortable').sortableTable()
  },
  methods: {
    start_new_deck() {
      event.preventDefault()
      this.mode = 'new'
    },

    create_deck(ev) {
      this.decks.push(ev.deck)
      this.mode = 'view'
    },

    update_deck(ev) {
      for (var i = 0; i < this.decks.length; i++) {
        if (this.decks[i].id == ev.deck.id) {
          Object.assign(this.decks[i], ev.deck)
          break
        }
      }
      this.mode = 'view'
    },

    delete_deck(id) {
      for (var i = 0; i < this.decks.length; i++) {
        if (this.decks[i].id == id) {
          this.decks.splice(i, 1)
          break
        }
      }
      this.mode = 'view'
    }
  }
}
</script>
