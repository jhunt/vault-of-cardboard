<template>
  <form :class="'goal data-entry ' + mode" @submit.prevent="save()">
    <h2 v-if="mode == 'new'">Set a New Goal</h2>
    <h2 v-else-if="mode == 'edit'">Update &ldquo;{{ title }}&rdquo;</h2>
    <div class="band">
      <div>
        <div class="control">
          <label for="name">Name</label>
           <input type="text" name="name" v-model="goal.name"
                  class="autofocus"
                  placeholder="A name for this goal"
                  data-error-if-missing="Please provide a name for this goal."
                  autocomplete="off">
        </div>
      </div>
    </div>
    <div class="band">
      <div>
        <div class="control">
          <label for="target">Target Query</label>
           <input type="text" name="target" v-model="goal.target"
                  class="autofocus"
                  placeholder="What are you hoping to collect?"
                  data-error-if-missing="Please provide a target query for this goal."
                  autocomplete="off">
        </div>
      </div>
    </div>
    <div class="band">
      <div>
        <div class="control">
          <label for="goal">Success Query</label>
           <input type="text" name="goal" v-model="goal.goal"
                  class="autofocus"
                  placeholder="When will you know you've succeeded?"
                  data-error-if-missing="Please provide a success query for this goal."
                  autocomplete="off">
        </div>
      </div>
    </div>

    <div>
      <button class="default safe" type="submit">{{ mode == 'new' ? "Set" : "Update" }} Goal</button>
      <button v-if="mode == 'edit'" @click.prevent="remove()" class="danger">Delete This Goal</button>
    </div>
  </form>
</template>

<script>
import { mapGetters } from 'vuex'
import cardboard from '@/lib/cardboard/index'

export default {
  name: 'vcb-goal-form',
  props: ['n', 'mode', 'object'],
  data: function() {
    let o = this.object || {}
    return {
      title: o.name || '',
      goal: {
        id:      o.id      || undefined,
        name:    o.name    || '',
        target:  o.target  || '',
        goal:    o.goal    || 'owned',
        ordinal: o.ordinal || this.n    || 0,
      }
    }
  },
  computed: {
    ...mapGetters(['session'])
  },
  methods: {
    save() {
      if (this.goal.id) {
        cardboard.API.patch_goal(this.session, this.goal)
          .then(data => this.$emit('updated-goal', data))

      } else {
        cardboard.API.post_goal(this.session, this.goal)
          .then(data => this.$emit('created-goal', data))
      }
    },
    remove() {
      cardboard.API.delete_goal(this.session, this.goal.id)
        .then(() => this.$emit('deleted-goal', this.goal.id))
    },
  }
}
</script>
