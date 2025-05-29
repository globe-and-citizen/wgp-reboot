<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {wasmBackend, getCookie} from '@/utils.js';

const poems = ref([]);

onMounted(() => {
  let token = getCookie('jwt') || "";

  wasmBackend.get_poems(null, token)
      .then(data => {
        let list = data.poems || data["poems"] || data.get("poems");

        for (let i = 0; i < list.length; i++) {
          let poem = {
            id: list[i].id || list[i]["id"] || list[i].get("id"),
            title: list[i].title || list[i]["title"] || list[i].get("title"),
            author: list[i].author || list[i]["author"] || list[i].get("author"),
            content: list[i].content || list[i]["content"] || list[i].get("content")
          }
          poems.value.push(poem);
        }
      }).catch(err => {
    console.error('Error fetching profile:', err);
  })
});
</script>

<template>
  <div class="poem-gallery">
    <h1>📖 Poem Collection</h1>
    <div class="poem-list">
      <div
          class="poem-card"
          v-for="poem in poems"
          :key="poem.id"
      >
        <h2>{{ poem.title }}</h2>
        <h3>by {{ poem.author }}</h3>
        <pre>{{ poem.content }}</pre>
      </div>
    </div>
  </div>
</template>

<style scoped>
.poem-gallery {
  max-width: 1200px;
  margin: auto;
  padding: 2rem;
}

h1 {
  text-align: center;
  margin-bottom: 2rem;
}

.poem-list {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.poem-card {
  background: #f9f9f9;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  transition: transform 0.2s;
}

.poem-card:hover {
  transform: scale(1.01);
}

.poem-card h2 {
  margin: 0 0 0.5rem 0;
  font-size: 1.3rem;
}

.poem-card h3 {
  margin: 0 0 1rem 0;
  font-weight: normal;
  color: #666;
}

.poem-card pre {
  white-space: pre-wrap;
  line-height: 1.6;
  font-family: inherit;
  color: #333;
}

/* Responsive layout */
@media (min-width: 768px) {
  .poem-list {
    flex-direction: row;
    flex-wrap: wrap;
    justify-content: center;
  }

  .poem-card {
    width: calc(33.33% - 2rem);
  }
}
</style>
