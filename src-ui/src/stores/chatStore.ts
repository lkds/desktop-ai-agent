import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'

interface Message {
  id: string
  role: 'user' | 'assistant' | 'system' | 'error'
  content: string
  timestamp: number
}

interface Step {
  id: string
  description: string
  status: 'pending' | 'running' | 'success' | 'failed'
  output?: string
}

interface Task {
  id: string
  description: string
  status: 'pending' | 'planning' | 'running' | 'completed' | 'failed' | 'cancelled'
  steps: Step[]
  result?: string
}

interface ChatState {
  messages: Message[]
  currentTask: Task | null
  isExecuting: boolean
  
  // Actions
  addMessage: (message: Message) => void
  clearMessages: () => void
  executeTask: (description: string) => Promise<void>
  pauseTask: () => Promise<void>
  cancelTask: () => Promise<void>
  updateTaskStatus: (task: Task) => void
}

export const useChatStore = create<ChatState>((set, get) => ({
  messages: [],
  currentTask: null,
  isExecuting: false,
  
  addMessage: (message) => {
    set(state => ({
      messages: [...state.messages, message]
    }))
  },
  
  clearMessages: () => {
    set({ messages: [], currentTask: null })
  },
  
  executeTask: async (description) => {
    // 添加用户消息
    get().addMessage({
      id: Date.now().toString(),
      role: 'user',
      content: description,
      timestamp: Date.now()
    })
    
    set({ isExecuting: true })
    
    try {
      // 调用 Tauri 后端执行任务
      const task = await invoke<Task>('execute_task', { description })
      
      // 添加系统消息
      get().addMessage({
        id: Date.now().toString(),
        role: 'system',
        content: '任务已开始执行...',
        timestamp: Date.now()
      })
      
      // 监听任务进度
      // TODO: 使用 Tauri event 监听进度更新
      
      set({ currentTask: task, isExecuting: false })
      
      // 添加结果消息
      if (task.result) {
        get().addMessage({
          id: Date.now().toString(),
          role: 'assistant',
          content: task.result.summary,
          timestamp: Date.now()
        })
      }
    } catch (error) {
      set({ isExecuting: false })
      get().addMessage({
        id: Date.now().toString(),
        role: 'error',
        content: `执行失败: ${error}`,
        timestamp: Date.now()
      })
    }
  },
  
  pauseTask: async () => {
    try {
      await invoke('pause_task')
      set(state => ({
        currentTask: state.currentTask ? { ...state.currentTask, status: 'paused' } : null
      }))
    } catch (error) {
      console.error('Pause failed:', error)
    }
  },
  
  cancelTask: async () => {
    try {
      await invoke('cancel_task')
      set({ currentTask: null, isExecuting: false })
    } catch (error) {
      console.error('Cancel failed:', error)
    }
  },
  
  updateTaskStatus: (task) => {
    set({ currentTask: task })
  }
}))