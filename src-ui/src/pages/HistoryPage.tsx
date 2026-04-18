import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api'

interface TaskHistory {
  id: string
  description: string
  status: string
  created_at: string
  result?: string
}

function HistoryPage() {
  const [history, setHistory] = useState<TaskHistory[]>([])
  
  useEffect(() => {
    loadHistory()
  }, [])
  
  const loadHistory = async () => {
    try {
      const tasks = await invoke<TaskHistory[]>('get_task_history')
      setHistory(tasks)
    } catch (error) {
      console.error('Load history failed:', error)
    }
  }
  
  return (
    <div className="history-container" style={{ padding: '20px' }}>
      <h2 style={{ marginBottom: '20px' }}>执行历史</h2>
      
      {history.map(task => (
        <div key={task.id} style={{
          background: 'var(--bg-light)',
          borderRadius: '8px',
          padding: '16px',
          marginBottom: '12px'
        }}>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span style={{ fontWeight: 500 }}>{task.description}</span>
            <span style={{
              fontSize: '12px',
              color: task.status === 'completed' ? 'var(--success)' : 
                     task.status === 'failed' ? 'var(--error)' : 'var(--text-muted)'
            }}>
              {task.status}
            </span>
          </div>
          
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '8px' }}>
            {task.created_at}
          </div>
          
          {task.result && (
            <div style={{ marginTop: '12px', fontSize: '14px' }}>
              {task.result}
            </div>
          )}
        </div>
      ))}
      
      {history.length === 0 && (
        <div style={{ color: 'var(--text-muted)' }}>
          暂无执行历史
        </div>
      )}
    </div>
  )
}

export default HistoryPage