// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getClientConfig } from '@/parsec/internals';
import { getCurrentAvailableDevice } from '@/parsec/login';
import {
  AdvisoryDeviceFilePrimaryProtection,
  DevicePrimaryProtectionStrategyTag,
  GetServerConfigError,
  Result,
  ServerConfig,
} from '@/parsec/types';
import { GetServerConfigErrorTag, libparsec, ParsecAddr, ParsedParsecAddr } from '@/plugins/libparsec';

export async function getServerConfig(serverAddr: string): Promise<Result<ServerConfig, GetServerConfigError>> {
  const STRATEGY_CONVERSION = new Map<DevicePrimaryProtectionStrategyTag, AdvisoryDeviceFilePrimaryProtection>([
    [DevicePrimaryProtectionStrategyTag.AccountVault, AdvisoryDeviceFilePrimaryProtection.AccountVault],
    [DevicePrimaryProtectionStrategyTag.Keyring, AdvisoryDeviceFilePrimaryProtection.Keyring],
    [DevicePrimaryProtectionStrategyTag.OpenBao, AdvisoryDeviceFilePrimaryProtection.OpenBao],
    [DevicePrimaryProtectionStrategyTag.PKI, AdvisoryDeviceFilePrimaryProtection.PKI],
    [DevicePrimaryProtectionStrategyTag.Password, AdvisoryDeviceFilePrimaryProtection.Password],
  ]);

  const result = await libparsec.getServerConfig(getClientConfig().configDir, serverAddr);

  if (result.ok) {
    if ((window as any).TESTING_MOCK_ALLOWED_PROTECTION_METHODS) {
      window.electronAPI.log('info', 'Mocking allowed protection methods');
      result.value.advisoryDeviceFileProtection = (window as any).TESTING_MOCK_ALLOWED_PROTECTION_METHODS;
    }

    (result.value as ServerConfig).isAuthMethodAllowed = (strategy: DevicePrimaryProtectionStrategyTag) => {
      return (
        result.value.advisoryDeviceFileProtection.length === 0 ||
        result.value.advisoryDeviceFileProtection.find((fileProtection) => fileProtection.primary === STRATEGY_CONVERSION.get(strategy)) !==
          undefined
      );
    };
    (result.value as ServerConfig).doesAuthMethodRequireTotp = (strategy: DevicePrimaryProtectionStrategyTag) => {
      return (
        result.value.advisoryDeviceFileProtection.find(
          (fileProtection) => fileProtection.primary === STRATEGY_CONVERSION.get(strategy) && fileProtection.withTotp,
        ) !== undefined
      );
    };
  }
  return result as Result<ServerConfig, GetServerConfigError>;
}

export async function getCurrentServerConfig(): Promise<Result<ServerConfig, GetServerConfigError>> {
  const result = await getCurrentServerAddr();
  if (result.ok) {
    return await getServerConfig(result.value);
  }
  return result;
}

export async function getCurrentServerAddr(): Promise<Result<ParsecAddr, GetServerConfigError>> {
  const result = await getCurrentAvailableDevice();
  if (!result.ok) {
    return { ok: false, error: { tag: GetServerConfigErrorTag.Internal, error: result.error.tag } };
  }
  return { ok: true, value: result.value.serverAddr };
}

export async function buildParsecAddr(addr: ParsedParsecAddr): Promise<string> {
  return await libparsec.buildParsecAddr(addr.hostname, addr.port, addr.useSsl);
}
