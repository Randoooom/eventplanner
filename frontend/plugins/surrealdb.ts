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

import { defineNuxtPlugin, useLocalePath, useRouter } from "#imports";
import { Surreal } from "surrealdb";
import { surrealdbWasmEngines } from "@surrealdb/wasm";
import { Ref, ref } from "#imports";

type Access = "account" | "visitor";

/**
 *
 */
export class SurrealdbConnection {
  public connection: Surreal;
  public loggedIn: Ref<boolean>;
  public access: Ref<Access | undefined>;

  /**
   *
   */
  constructor() {
    this.connection = new Surreal({ engines: surrealdbWasmEngines() });
    this.loggedIn = ref(false);
    this.access = ref(undefined);
  }

  /**
   *
   * @param surrealdbEndpoint
   */
  async init(surrealdbEndpoint: string) {
    await this.connection.connect(surrealdbEndpoint);
  }

  /**
   *
   * @param username
   * @param password
   */
  async signup(username: string, password: string) {
    const token = await this.connection.signup({
      variables: {
        username,
        password
      },
      namespace: "production",
      database: "eventplanner",
      access: "account",
    });
    await this.connection.authenticate(token);
    await this.connection.use({ namespace: "production", database: "eventplanner" })

    this.access.value = "account";
    this.loggedIn.value = true;
  }

  /**
   *
   * @param username
   * @param password
   */
  async login(username: string, password: string) {
    const token = await this.connection.signin({
      variables: {
        username,
        password
      },
      namespace: "production",
      database: "eventplanner",
      access: "account",
    });
    await this.connection.authenticate(token);
    await this.connection.use({ namespace: "production", database: "eventplanner" })

    this.access.value = "account";
    this.loggedIn.value = true;
  }

  /**
   *
   * @param id
   */
  async visit(id: string) {
    const token = await this.connection.signin({
      variables: {
        id
      },
      namespace: "production",
      database: "eventplanner",
      access: "visitor",
    });
    await this.connection.authenticate(token);
    await this.connection.use({ namespace: "production", database: "eventplanner" })
    this.access.value = "visitor";
    this.loggedIn.value = true;
  }

  /**
   *
   */
  async logout() {
    await this.connection.invalidate();
    this.loggedIn.value = false;

    const localePath = useLocalePath();
    await useRouter().push(localePath("/"));
  }
}

export default defineNuxtPlugin(async (app) => {
  const connection = new SurrealdbConnection();

  // connect to the database
  await connection.connection.connect(app.$config.public.surrealdbEndpoint);

  return {
    provide: {
      surrealdb: connection,
    },
  };
});
