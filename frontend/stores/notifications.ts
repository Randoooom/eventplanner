/*
 *     Copyright (C) 2023 Fritz Ochsmann
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU Affero General Public License as published
 *     by the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU Affero General Public License for more details.
 *
 *     You should have received a copy of the GNU Affero General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { defineStore } from "pinia";
import { nanoid } from "nanoid";

export interface CreateNotification {
  title: string;
  content?: string;
  timeout?: number;
  color?: string;
  icon?: string;
}

export interface Notification extends CreateNotification {
  created: Date;
  id: string;
}

export const notificationEmitter = defineStore("notifications", {
  state: () => ({
    openNotifications: [] as Notification[],
  }),
  actions: {
    removeNotification(id: string) {
      this.openNotifications = this.openNotifications.filter(
        (notification: Notification) => notification.id !== id
      );
    },
    attachNotification(create: CreateNotification) {
      const notification: Notification = {
        created: new Date(),
        id: nanoid(),
        title: create.title,
        timeout: create.timeout || 5000,
        content: create.content,
        color: create.color || "info",
        icon: create.icon || "mdi-information-outline",
      };

      this.openNotifications.push(notification);
      if (notification.timeout)
        setTimeout(
          () => this.removeNotification(notification.id),
          notification.timeout
        );

      return notification;
    },
  },
});
