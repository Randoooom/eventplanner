<!--
  -     Copyright (C) 2023 Fritz Ochsmann
  -
  -     This program is free software: you can redistribute it and/or modify
  -     it under the terms of the GNU Affero General Public License as published
  -     by the Free Software Foundation, either version 3 of the License, or
  -     (at your option) any later version.
  -
  -     This program is distributed in the hope that it will be useful,
  -     but WITHOUT ANY WARRANTY; without even the implied warranty of
  -     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  -     GNU Affero General Public License for more details.
  -
  -     You should have received a copy of the GNU Affero General Public License
  -     along with this program.  If not, see <https://www.gnu.org/licenses/>.
  -->

<template>
  <v-fade-transition>
    <v-alert
      @click:close="notifications.removeNotification(notification.id)"
      class="notification"
      v-for="notification in notifications.openNotifications"
      :key="notification.id"
      :icon="notification.icon"
      :color="notification.color"
      :text="notification.content ? $t(notification.content) : undefined"
      :title="$t(notification.title)"
      closable
      position="fixed"
    />
  </v-fade-transition>
</template>

<script setup lang="ts">
import { notificationEmitter } from "~/stores/notifications";

const notifications = notificationEmitter();
</script>

<style lang="sass" scoped>
.notification
  position: fixed
  bottom: 50px
  left: 50%
  transform: translateX(-50%)

  @media screen and (max-width: 600px)
    width: calc(100vw - 18px)
</style>
