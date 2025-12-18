// Auto-update checker component

import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { Download, X, RefreshCw } from 'lucide-react';
import { check, type DownloadEvent } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

interface UpdateInfo {
  version: string;
  body?: string;
}

export function UpdateChecker() {
  const { t } = useTranslation();
  const [updateAvailable, setUpdateAvailable] = useState<UpdateInfo | null>(null);
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [isInstalling, setIsInstalling] = useState(false);
  const [dismissed, setDismissed] = useState(false);

  useEffect(() => {
    checkForUpdates();
  }, []);

  const checkForUpdates = async () => {
    try {
      const update = await check();
      if (update?.available) {
        setUpdateAvailable({
          version: update.version,
          body: update.body,
        });
      }
    } catch (error) {
      console.error('Failed to check for updates:', error);
    }
  };

  const downloadAndInstall = async () => {
    if (!updateAvailable) return;

    try {
      setIsDownloading(true);
      const update = await check();

      if (update?.available) {
        await update.downloadAndInstall((progress: DownloadEvent) => {
          if (progress.event === 'Progress') {
            const data = progress.data as { chunkLength?: number; contentLength?: number };
            const percent = Math.round(
              ((data.chunkLength || 0) / (data.contentLength || 1)) * 100
            );
            setDownloadProgress(percent);
          }
        });

        setIsDownloading(false);
        setIsInstalling(true);

        // Relaunch the app to apply the update
        await relaunch();
      }
    } catch (error) {
      console.error('Failed to download update:', error);
      setIsDownloading(false);
    }
  };

  if (!updateAvailable || dismissed) return null;

  return (
    <AnimatePresence>
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
    </AnimatePresence>
  );
}
