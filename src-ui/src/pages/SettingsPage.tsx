import { useState, useEffect } from 'react'
import { useConfigStore } from '../stores/configStore'
import { FolderOpen, Plus, Trash2 } from 'lucide-react'

function SettingsPage() {
  const {
    providers,
    currentProvider,
    currentModel,
    allowedPaths,
    loadConfig,
    setProvider,
    addAllowedPath,
    removeAllowedPath
  } = useConfigStore()
  
  const [apiKey, setApiKey] = useState('')
  const [baseUrl, setBaseUrl] = useState('')
  const [model, setModel] = useState('')
  const [providerKind, setProviderKind] = useState<'openai' | 'claude' | 'ollama' | 'custom'>('openai')
  
  useEffect(() => {
    loadConfig()
  }, [])
  
  const handleSaveProvider = () => {
    const provider = {
      kind: providerKind,
      api_key: apiKey,
      base_url: baseUrl || undefined,
      model
    }
    setProvider(provider)
  }
  
  return (
    <div className="settings-container">
      {/* Model Provider Settings */}
      <section className="settings-section">
        <h2>模型配置</h2>
        
        <div className="settings-item">
          <label>Provider 类型</label>
          <select value={providerKind} onChange={e => setProviderKind(e.target.value as any)}>
            <option value="openai">OpenAI</option>
            <option value="claude">Claude</option>
            <option value="ollama">Ollama (本地)</option>
            <option value="custom">自定义</option>
          </select>
        </div>
        
        <div className="settings-item">
          <label>API Key</label>
          <input
            type="password"
            value={apiKey}
            onChange={e => setApiKey(e.target.value)}
            placeholder={providerKind === 'ollama' ? '本地模型无需 API Key' : '输入 API Key'}
            disabled={providerKind === 'ollama'}
          />
        </div>
        
        <div className="settings-item">
          <label>Base URL</label>
          <input
            type="text"
            value={baseUrl}
            onChange={e => setBaseUrl(e.target.value)}
            placeholder={getDefaultBaseUrl(providerKind)}
          />
        </div>
        
        <div className="settings-item">
          <label>模型名称</label>
          <input
            type="text"
            value={model}
            onChange={e => setModel(e.target.value)}
            placeholder={getDefaultModel(providerKind)}
          />
        </div>
        
        <button onClick={handleSaveProvider} style={{
          padding: '12px 24px',
          background: 'var(--primary)',
          color: 'white',
          border: 'none',
          borderRadius: '8px',
          cursor: 'pointer'
        }}>
          保存配置
        </button>
        
        {currentModel && (
          <div style={{ marginTop: '16px', color: 'var(--text-muted)' }}>
            当前使用: {currentModel}
          </div>
        )}
      </section>
      
      {/* File Permissions */}
      <section className="settings-section">
        <h2>文件访问权限</h2>
        
        <div style={{ marginBottom: '16px' }}>
          {allowedPaths.map(path => (
            <div key={path} className="settings-item">
              <FolderOpen size={16} />
              <span style={{ flex: 1 }}>{path}</span>
              <button onClick={() => removeAllowedPath(path)} style={{
                background: 'transparent',
                border: 'none',
                color: 'var(--error)',
                cursor: 'pointer'
              }}>
                <Trash2 size={16} />
              </button>
            </div>
          ))}
          
          {allowedPaths.length === 0 && (
            <div style={{ color: 'var(--text-muted)' }}>
              未设置允许访问的路径
            </div>
          )}
        </div>
        
        <div className="settings-item">
          <input
            type="text"
            placeholder="添加路径..."
            id="new-path-input"
          />
          <button onClick={() => {
            const input = document.getElementById('new-path-input') as HTMLInputElement
            if (input.value.trim()) {
              addAllowedPath(input.value.trim())
              input.value = ''
            }
          }} style={{
            background: 'var(--primary)',
            color: 'white',
            border: 'none',
            borderRadius: '6px',
            padding: '8px 12px',
            cursor: 'pointer'
          }}>
            <Plus size={16} />
          </button>
        </div>
      </section>
    </div>
  )
}

function getDefaultBaseUrl(kind: string): string {
  switch (kind) {
    case 'openai': return 'https://api.openai.com/v1'
    case 'claude': return 'https://api.anthropic.com/v1'
    case 'ollama': return 'http://localhost:11434/v1'
    default: return ''
  }
}

function getDefaultModel(kind: string): string {
  switch (kind) {
    case 'openai': return 'gpt-4'
    case 'claude': return 'claude-3-opus'
    case 'ollama': return 'llama3'
    default: return ''
  }
}

export default SettingsPage