import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/components/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/poems',
      name: 'poems',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('@/components/GetPoems.vue'),
    },
    {
      path: '/pictures',
      name: 'pictures',
      component: () => import('@/components/GetPictures.vue'),
    },
    {
      path: '/profile',
      name: 'profile',
      component: () => import('@/components/Profile.vue'),
    }
  ],
})

export default router
