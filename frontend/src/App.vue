<script setup lang="ts">
import {RouterLink, RouterView} from 'vue-router'
import {computed} from "vue";
import {getCookie} from "@/utils.ts";

const isLoggedIn = computed(() => {
  return getCookie("jwt") !== undefined && getCookie("jwt")?.length > 0; // todo
})

function logout() {
  document.cookie = 'jwt=; expires=Thu, 01 Jan 1970 00:00:01 GMT;';
  alert('Logged out successfully.')
  location.href = '/'
}
</script>

<template>
  <div style="position: relative; min-height: 4rem;">
    <nav>
      <RouterLink v-if="!isLoggedIn" to="/">Home</RouterLink>
      <RouterLink v-if="!isLoggedIn" to="/register">Register</RouterLink>
      <div v-if="isLoggedIn">
        <RouterLink to="/poems">Poems</RouterLink>
        <RouterLink to="/pictures">Pictures</RouterLink>
        <RouterLink to="/profile">Profile</RouterLink>
        <a href="/" @click.prevent="logout" style="padding:0 1rem;cursor:pointer;">Logout</a>
      </div>
    </nav>
  </div>

  <div style="height: 4rem;"></div>

  <div class="body">
    <RouterView/>
  </div>

</template>

<style scoped>
nav {
  width: 100%;
  font-size: 1rem;
  text-align: right;
  padding: 1rem 0;
  margin-top: 0;
  position: static;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  nav {
    width: 100%;
    font-size: 1rem;
    text-align: right;
    padding: 1rem 0;
    margin-top: 0;
    position: static;
  }
}
</style>

