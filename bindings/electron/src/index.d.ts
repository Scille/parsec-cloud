// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */


export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E }


export interface ClientConfig {
    configDir: string
    dataBaseDir: string
    mountpointBaseDir: string
    preferredOrgCreationBackendAddr: string
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
}


// WorkspaceStorageCacheSize
export interface WorkspaceStorageCacheSizeCustom {
    tag: "Custom"
    size: number
}
export interface WorkspaceStorageCacheSizeDefault {
    tag: "Default"
}
export type WorkspaceStorageCacheSize =
  | WorkspaceStorageCacheSizeCustom
  | WorkspaceStorageCacheSizeDefault


export function testNewTestbed(
    template: string,
    test_server: string | null
): Promise<string>
export function testDropTestbed(
    path: string
): Promise<null>
