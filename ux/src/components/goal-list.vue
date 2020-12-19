<template>
  <div class="goals listing prose">
    <h1>It's Good To Have Goals</h1>
    <div class="buttons" v-if="mode == 'view'">
      <button class="action" @click="start_new_goal()">Set New Goal</button>
    </div>
    <vcb-goal-form v-else-if="mode == 'edit'"
      :object="goal"
      @updated-goal="update_goal($event)"
      mode="edit"></vcb-goal-form>
    <vcb-goal-form
      v-else
      :n="goals.length"
      @created-goal="create_goal($event)"
      mode="new"></vcb-goal-form>

    <table class="goals sortable listing">
      <thead><tr>
        <th class="sortable">Name</th>
        <th>Target</th>
        <th></th>
        <th>Goal</th>
        <th>Progress</th>
        <th class="sortable" data-sort-as="number">Created</th>
        <th class="sortable" data-sort-as="number">Last Updated</th>
        <th></th>
      </tr></thead>
      <tbody>
        <tr v-for="goal in goals" :key="goal.id">
          <td>{{ goal.name }}</td>
          <td>(<router-link :to='{ name: "search", params: { query: query(goal.target) }}' tag="tt">{{ goal.target }}</router-link>)</td>
          <td>and</td>
          <td>(<router-link :to='{ name: "search", params: { query: query(goal.target, goal.goal) }}' tag="tt">{{ goal.goal }}</router-link>)</td>
          <td>{{ acheived(goal) }}/{{ total(goal) }} ({{ parseInt(acheived(goal) * 100 / total(goal)) }}%)</td>
          <td :data-sort="goal.created_at">{{ strftime("%B %Oe, %Y", dated(goal.created_at)) }}</td>
          <td :data-sort="goal.updated_at">{{ strftime("%B %Oe, %Y", dated(goal.updated_at)) }}</td>
          <td><a href="#" @click="edit_goal(goal)">edit</a></td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import VcbGoalForm from '@/components/goal-form'

export default {
  name: 'goal-list',
  components: {
    VcbGoalForm
  },
  props: {
    goals: Array
  },
  data: function () {
    return {
      mode: 'view',
      goal: undefined,

      targets:  {},
      progress: {}
    }
  },
  computed: {
    ...mapGetters(['vault'])
  },
  methods: {
    query(target, goal) {
      return goal ? `(${target}) and (${goal})` : target
    },

    total(goal) {
      if (!(goal.id in this.targets)) {
        let l = this.vault.search(this.query(goal.target))
        this.targets[goal.id] = l.length
      }
      return this.targets[goal.id]
    },
    acheived(goal) {
      if (!(goal.id in this.progress)) {
        let l = this.vault.search(this.query(goal.target, goal.goal))
        this.progress[goal.id] = l.length
      }
      return this.progress[goal.id]
    },

    start_new_goal() {
      event.preventDefault()
      this.mode = 'new'
    },

    show_goal(goal) {
      this.goal = goal
    },

    create_goal(ev) {
      this.goals.push(ev.goal)
      this.mode = 'view'
    },

    edit_goal(goal) {
      this.mode = 'edit'
      this.goal = goal
    },

    update_goal(ev) {
      for (var i = 0; i < this.goals.length; i++) {
        if (this.goals[i].id == ev.goal.id) {
          Object.assign(this.goals[i], ev.goal)
          break
        }
      }
      this.mode = 'view'
    },

    delete_goal(id) {
      for (var i = 0; i < this.goals.length; i++) {
        if (this.goals[i].id == id) {
          this.goals.splice(i, 1)
          break
        }
      }
      this.mode = 'view'
    }
  }
}
</script>
