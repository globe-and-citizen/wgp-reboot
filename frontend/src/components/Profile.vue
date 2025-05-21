<script setup>
const profile = {
  name: "ChatGPT",
  title: "AI Assistant by OpenAI",
  avatar: "https://upload.wikimedia.org/wikipedia/commons/0/04/ChatGPT_logo.svg",
  bio: `ChatGPT is a language model designed to assist with writing, coding, learning, and more.
Trained on a wide range of data, it aims to provide accurate, clear, and human-like responses to support users in diverse tasks.`,
  email: "Not applicable 😊",
  location: "The Cloud ☁️",
  website: "https://openai.com/chatgpt"
}
</script>

<template>
  <div class="profile-container">
    <div class="profile-card">
      <img class="avatar" :src="profile.avatar" :alt="profile.name" />
      <h1>{{ profile.name }}</h1>
      <h2>{{ profile.title }}</h2>
      <p class="bio">{{ profile.bio }}</p>
      <div class="contact">
        <p><strong>📍 Location:</strong> {{ profile.location }}</p>
        <p><strong>✉️ Email:</strong> <a :href="`mailto:${profile.email}`">{{ profile.email }}</a></p>
        <p><strong>🌐 Website:</strong> <a :href="profile.website" target="_blank">{{ profile.website }}</a></p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.profile-container {
  display: flex;
  justify-content: center;
  padding: 3rem 1rem;
  min-height: 100vh;
}

.profile-card {
  background: #fff;
  padding: 2rem;
  border-radius: 1rem;
  max-width: 500px;
  text-align: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.avatar {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  object-fit: cover;
  margin-bottom: 1rem;
}

h1 {
  margin: 0.5rem 0 0.2rem;
  font-size: 1.8rem;
}

h2 {
  margin: 0 0 1.5rem;
  font-size: 1.1rem;
  color: #777;
}

.bio {
  font-size: 1rem;
  color: #333;
  margin-bottom: 1.5rem;
  white-space: pre-line;
}

.contact p {
  font-size: 0.95rem;
  margin: 0.5rem 0;
}

.contact a {
  color: #0077cc;
  text-decoration: none;
}
.contact a:hover {
  text-decoration: underline;
}
</style>
