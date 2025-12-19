// Auto-update checker component

import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { Download, X, RefreshCw, AlertCircle, CheckCircle } from 'lucide-react';
import { check, type DownloadEvent } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { getVersion } from '@tauri-apps/api/app';

interface UpdateInfo {
  version: string;
  body?: string;
}

type CheckStatus = 'idle' | 'checking' | 'error' | 'no-update' | 'available';

export function UpdateChecker() {
  const { t } = useTranslation();
  const [currentVersion, setCurrentVersion] = useState<string>('');
  const [updateAvailable, setUpdateAvailable] = useState<UpdateInfo | null>(null);
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [isInstalling, setIsInstalling] = useState(false);
  const [dismissed, setDismissed] = useState(false);
  const [checkStatus, setCheckStatus] = useState<CheckStatus>('idle');
  const [errorMessage, setErrorMessage] = useState<string>('');
  const [showDebug, setShowDebug] = useState(false);

  useEffect(() => {
    // Get current version
    getVersion().then(setCurrentVersion).catch(() => {});

    // Delay check to ensure app is fully loaded
    const timer = setTimeout(() => {
      checkForUpdates();
    }, 2000);
    return () => clearTimeout(timer);
  }, []);

  const checkForUpdates = async () => {
    try {
      setCheckStatus('checking');
      setErrorMessage('');
      const update = await check();
      if (update?.available) {
        setCheckStatus('available');
        setUpdateAvailable({
          version: update.version,
          body: update.body,
        });
      } else {
        setCheckStatus('no-update');
      }
    } catch (error) {
      setCheckStatus('error');
      setErrorMessage(String(error));
    }
  };

  const downloadAndInstall = async () => {
    if (!updateAvailable) return;

    try {
      setIsDownloading(true);
      const update = await check();

      if (update?.available) {
        let downloaded = 0;
        await update.downloadAndInstall((progress: DownloadEvent) => {
          if (progress.event === 'Progress') {
            const data = progress.data as { chunkLength?: number; contentLength?: number };
            downloaded += data.chunkLength || 0;
            const total = data.contentLength || 1;
            const percent = Math.min(100, Math.round((downloaded / total) * 100));
            setDownloadProgress(percent);
          }
        });

        setIsDownloading(false);
        setIsInstalling(true);

        // Relaunch the app to apply the update
        await relaunch();
      }
    } catch (error) {
      setIsDownloading(false);
      setErrorMessage(String(error));
    }
  };

  return (
    <>
      {/* Debug indicator - bottom left */}
      <div className="fixed bottom-4 left-4 z-50">
        <button
          onClick={() => setShowDebug(!showDebug)}
          className={`px-2 py-1 rounded text-xs font-mono ${
            checkStatus === 'error' ? 'bg-red-600' :
            checkStatus === 'available' ? 'bg-green-600' :
            checkStatus === 'checking' ? 'bg-yellow-600' :
            'bg-gray-700'
          } text-white`}
        >
          v{currentVersion || '?'}
        </button>

        {showDebug && (
          <div className="absolute bottom-8 left-0 bg-gray-800 border border-gray-600 rounded-lg p-3 min-w-[250px] text-xs">
            <div className="space-y-1">
              <p className="text-gray-400">Current: <span className="text-white">v{currentVersion}</span></p>
              <p className="text-gray-400">Status: <span className={
                checkStatus === 'error' ? 'text-red-400' :
                checkStatus === 'available' ? 'text-green-400' :
                checkStatus === 'checking' ? 'text-yellow-400' :
                'text-white'
              }>{checkStatus}</span></p>
              {updateAvailable && (
                <p className="text-gray-400">Latest: <span className="text-green-400">v{updateAvailable.version}</span></p>
              )}
              {errorMessage && (
                <p className="text-red-400 break-all">{errorMessage}</p>
              )}
              <button
                onClick={checkForUpdates}
                className="mt-2 px-2 py-1 bg-blue-600 hover:bg-blue-500 rounded text-white w-full"
              >
                Check Again
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Update notification */}
      <AnimatePresence>
        {updateAvailable && !dismissed && (
          <motion.div
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            className="fixed top-4 right-4 z-50 bg-gray-800 border border-gray-700 rounded-lg shadow-xl p-4 max-w-sm"
          >
            <div className="flex items-start gap-3">
              <div className="p-2 bg-blue-600/20 rounded-lg shrink-0">
                <Download className="w-5 h-5 text-blue-400" />
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="font-medium text-white">
                  {t('update.available', 'Update Available')}
                </h3>
                <p className="text-sm text-gray-400 mt-1">
                  {t('update.version', 'Version {{version}} is available', {
                    version: updateAvailable.version,
                  })}
                </p>

                {isDownloading && (
                  <div className="mt-3">
                    <div className="h-2 bg-gray-700 rounded-full overflow-hidden">
                      <motion.div
                        className="h-full bg-blue-500"
                        initial={{ width: 0 }}
                        animate={{ width: `${downloadProgress}%` }}
                      />
                    </div>
                    <p className="text-xs text-gray-500 mt-1">
                      {t('update.downloading', 'Downloading...')} {downloadProgress}%
                    </p>
                  </div>
                )}

                {isInstalling && (
                  <div className="mt-3 flex items-center gap-2 text-sm text-blue-400">
                    <RefreshCw className="w-4 h-4 animate-spin" />
                    {t('update.installing', 'Installing update...')}
                  </div>
                )}

                {!isDownloading && !isInstalling && (
                  <div className="mt-3 flex gap-2">
                    <button
                      onClick={downloadAndInstall}
                      className="px-3 py-1.5 bg-blue-600 hover:bg-blue-500 text-white text-sm rounded-lg transition-colors"
                    >
                      {t('update.install', 'Install Now')}
                    </button>
                    <button
                      onClick={() => setDismissed(true)}
                      className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 text-gray-300 text-sm rounded-lg transition-colors"
                    >
                      {t('update.later', 'Later')}
                    </button>
                  </div>
                )}
              </div>

              {!isDownloading && !isInstalling && (
                <button
                  onClick={() => setDismissed(true)}
                  className="text-gray-500 hover:text-gray-400 transition-colors"
                >
                  <X className="w-4 h-4" />
                </button>
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </>
  );
}
