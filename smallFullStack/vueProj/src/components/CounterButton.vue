<script setup>
import { defineProps, ref } from 'vue';
const props = defineProps(['word'])
const count = ref(0)
</script>

<template>
    <button class="btn" @click="count++">We have: {{ count }} {{ word }}s!</button>
</template>

