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
  <v-footer class="d-flex text-center flex-column pt-8">
    <v-row class="justify-space-between">
      <v-col cols="4">
        <a href="https://github.com/randoooom/eventplaner" target="_blank">
          GitHub
        </a>
      </v-col>
    </v-row>

    <v-row class="pa-5">
      <v-col cols="12">
        &copy; Fritz Ochsmann - {{ new Date().getFullYear() }}
      </v-col>
    </v-row>
  </v-footer>
</template>

<style lang="sass" scoped>
.v-footer
  width: 100%
  background: linear-gradient(180deg, rgba(140, 158, 255, 1) 0%, rgba(70, 81, 185, 1) 100%)
</style>
