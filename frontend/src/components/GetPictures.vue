<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {wasmBackend, getCookie} from '@/utils.js';

const images = ref([]);

onMounted(() => {
  let token = getCookie('jwt') || "";

  wasmBackend.get_images(null, token)
      .then(data => {
        let list = data.images || data["images"] || data.get("images");

        for (let i = 0; i < list.length; i++) {
          let image = {
            id: list[i].id || list[i]["id"] || list[i].get("id"),
            title: list[i].title || list[i]["title"] || list[i].get("title"),
            content: list[i].content || list[i]["content"] || list[i].get("content"),
          }
          images.value.push(image);
        }
      }).catch(err => {
    console.error('Error fetching profile:', err);
  })
});
</script>

<template>
  <div class="gallery-vertical">
    <h1>📸 Image Gallery</h1>
    <div
        class="image-card"
        v-for="image in images"
        :key="image.id"
    >
      <img :src="image.src" :alt="image.title" />
      <h2>{{ image.title }}</h2>
    </div>
  </div>
</template>

<style scoped>
.gallery-vertical {
  max-width: 600px;
  margin: 2rem auto;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.image-card {
  margin-bottom: 2rem;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  width: 100%;
}

.image-card img {
  width: 100%;
  height: auto;
  object-fit: cover;
}

.image-card h2 {
  padding: 1rem;
  font-size: 1.1rem;
  text-align: center;
}
</style>
