<script setup>
const images = [
  {
    id: 1,
    title: "Sunset Over the Hills",
    src: "https://live.staticflickr.com/65535/50708980327_3b29ff3e65_h.jpg"
  },
  {
    id: 2,
    title: "Mountain Range",
    src: "https://live.staticflickr.com/65535/50729525272_92a0202f37_h.jpg"
  },
  {
    id: 3,
    title: "City Skyline",
    src: "https://live.staticflickr.com/65535/50722509432_44aa969d6a_h.jpg"
  },
  {
    id: 4,
    title: "Forest Path",
    src: "https://live.staticflickr.com/65535/50690937702_c1e765dda2_h.jpg"
  },
  {
    id: 5,
    title: "Ocean Waves",
    src: "https://live.staticflickr.com/65535/50344935577_10dc23e71b_h.jpg"
  }
]
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
