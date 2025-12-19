// Chat component

import { useState, useRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Bot, User, Send, Loader2, Trash2 } from 'lucide-react';
import { useChatStore, Message } from '../stores/chatStore';
import { useShallow } from 'zustand/shallow';

export function Chat() {
  const { t } = useTranslation();
  const [input, setInput] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const { messages, isProcessing, sendMessage, clearMessages } = useChatStore(
    useShallow((state) => ({
      messages: state.messages,
      isProcessing: state.isProcessing,
      sendMessage: state.sendMessage,
      clearMessages: state.clearMessages,
    }))
  );

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || isProcessing) return;

    const message = input.trim();
    setInput('');
    await sendMessage(message);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e);
    }
  };

  return (
    <div className="flex flex-col h-full">
      {/* Messages Area */}
      <div className="flex-1 overflow-y-auto p-6">
        <div className="max-w-3xl mx-auto space-y-4">
          {messages.length === 0 ? (
            <WelcomeMessage />
          ) : (
            messages.map((message) => (
              <MessageBubble key={message.id} message={message} />
            ))
          )}

          {isProcessing && (
            <div className="flex items-start gap-4">
              <div className="p-2 bg-ai-glow/20 rounded-lg shrink-0">
                <Bot className="w-5 h-5 text-ai-glow" />
              </div>
              <div className="bg-gray-800 rounded-lg p-4 flex items-center gap-2">
                <Loader2 className="w-4 h-4 animate-spin text-ai-glow" />
                <span className="text-gray-400">{t('chat.processing')}</span>
              </div>
            </div>
          )}

          <div ref={messagesEndRef} />
        </div>
      </div>

      {/* Input Area */}
      <div className="border-t border-gray-800 p-4">
        <div className="max-w-3xl mx-auto">
          <form onSubmit={handleSubmit} className="flex items-center gap-3">
            {messages.length > 0 && (
              <button
                type="button"
                onClick={clearMessages}
                className="p-2 text-gray-500 hover:text-gray-300 transition-colors"
                title={t('chat.clear')}
              >
                <Trash2 className="w-5 h-5" />
              </button>
            )}

            <div className="flex-1 relative">
              <input
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder={t('input.placeholder')}
                disabled={isProcessing}
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-3 pr-12
                         text-white placeholder-gray-500 focus:outline-none focus:border-ai-glow
                         focus:ring-1 focus:ring-ai-glow transition-colors
                         disabled:opacity-50 disabled:cursor-not-allowed"
              />
              <button
                type="submit"
                disabled={!input.trim() || isProcessing}
                className="absolute right-3 top-1/2 -translate-y-1/2 p-1.5
                         bg-ai-glow hover:bg-ai-glow/80 rounded-md transition-colors
                         disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <Send className="w-4 h-4 text-white" />
              </button>
            </div>
          </form>

          <p className="text-gray-600 text-xs mt-2 text-center">
            {isProcessing ? t('input.processing') : t('input.status')}
          </p>
        </div>
      </div>
    </div>
  );
}

function WelcomeMessage() {
  const { t } = useTranslation();

  return (
    <div className="flex items-start gap-4">
      <div className="p-2 bg-ai-glow/20 rounded-lg shrink-0">
        <Bot className="w-5 h-5 text-ai-glow" />
      </div>
      <div className="bg-gray-800 rounded-lg p-4 flex-1">
        <p className="text-gray-300">{t('welcome.message')}</p>
        <p className="text-gray-500 text-sm mt-2">{t('welcome.hint')}</p>
      </div>
    </div>
  );
}

function MessageBubble({ message }: { message: Message }) {
  const isUser = message.role === 'user';

  return (
    <div className={`flex items-start gap-4 ${isUser ? 'flex-row-reverse' : ''}`}>
      <div
        className={`p-2 rounded-lg shrink-0 ${
          isUser ? 'bg-gray-700' : 'bg-ai-glow/20'
        }`}
      >
        {isUser ? (
          <User className="w-5 h-5 text-gray-300" />
        ) : (
          <Bot className="w-5 h-5 text-ai-glow" />
        )}
      </div>

      <div
        className={`rounded-lg p-4 max-w-[80%] ${
          isUser ? 'bg-ai-glow/20' : 'bg-gray-800'
        }`}
      >
        <p className="text-gray-300 whitespace-pre-wrap selectable">{message.content}</p>

        {message.action && message.action.action !== 'none' && (
          <div className="mt-3 pt-3 border-t border-gray-700">
            <p className="text-xs text-gray-500 mb-1">Action:</p>
            <code className="text-xs text-ai-glow selectable">
              {message.action.action}({JSON.stringify(message.action.params)})
            </code>
          </div>
        )}
      </div>
    </div>
  );
}
