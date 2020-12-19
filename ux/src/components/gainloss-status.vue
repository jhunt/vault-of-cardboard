<template>
  <div class="status">
    <span v-if="status == 'working'" class="in-progress">⌛ validating these cards...</span>
    <span v-else-if="status == 'ok'" class="ok">👍 everything checks out with these cards!</span>
  </div>
</template>

<script>
export default {
  name: 'vcb-gainloss-status',
  props: ['status'],
}
</script>
