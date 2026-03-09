# ✅ TASKMASTER — Decentralized Todo List on Solana

Un gestor de tareas persistente y resistente a la censura construido sobre la blockchain de Solana utilizando **Anchor Framework**.

---

# ✨ Descripción General

TASKMASTER permite a los usuarios gestionar su productividad directamente **on-chain**. A diferencia de las aplicaciones tradicionales, aquí tú eres el único dueño de tus datos, almacenados de forma segura en **Program Derived Addresses (PDAs)** vinculadas a tu firma electrónica.

---

# 🧩 Características Principales

## 📝 Gestión de Tareas
Los usuarios pueden crear, marcar como completadas y eliminar tareas. Cada tarea incluye:
- **Descripción**: Detalle de la actividad (máx 50 caracteres).
- **Estado**: Booleano para verificar si la tarea ha sido finalizada.
- **Timestamp**: Fecha de creación registrada por la red.

## 🔒 Privacidad por Diseño
Cada lista de tareas está vinculada a una PDA derivada de la llave pública del usuario:
```rust
["todo_list", owner_pubkey, task_id]
