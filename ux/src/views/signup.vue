<template>
  <vcb-signup @redirect-signin="goto('/signin')"></vcb-signup>
</template>

<script>
import VcbSignup from '@/components/signup'

export default {
  components: {VcbSignup}
}
</script>
