<template>
  <vcb-signin @redirect-signup="goto('/signup')"></vcb-signin>
</template>

<script>
import VcbSignin from '@/components/signin'

export default {
  components: {VcbSignin}
}
</script>
