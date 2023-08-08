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
  <v-app-bar class="overflow-visible">
    <v-container>
      <v-row class="align-center">
        <v-app-bar-title class="d-flex align-center justify-start">
          <a href="https://github.com/randoooom/eventplaner" target="_blank">
            <v-icon icon="mdi-github" />
          </a>
        </v-app-bar-title>

        <v-spacer />

        <LayoutAuthentication />
      </v-row>
    </v-container>
  </v-app-bar>
</template>

<style lang="sass" scoped>
.v-app-bar
  border-bottom: 2px solid rgb(var(--v-theme-accent))

  .v-container
    max-width: 1200px
</style>
<script setup lang="ts"></script>
