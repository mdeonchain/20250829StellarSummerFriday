# 🧩 Contratos de Ejemplo – Soroban & Stellar

Este directorio contiene contratos de ejemplo en **Rust** para comprender la lógica básica y las operaciones fundamentales en **Soroban**, el entorno de smart contracts de **Stellar**.

---

## ⚙️ Entorno de desarrollo

Antes de comenzar, asegúrate de contar con un entorno configurado correctamente.  
Puedes usar **GitHub Codespaces**, que te permite trabajar directamente en la nube ☁️ con un entorno similar a **VS Code**, sin instalar nada localmente.

1. Abre el repositorio en GitHub.  
2. Haz clic en **Code → Codespaces → Create codespace on main**.  
3. ¡Listo! Tendrás un entorno listo para compilar, probar y desplegar contratos.

![](../images/codespaces.png)

---

  
👉 ¡Y listo! Tienes un **VS Code en el navegador** 🌐⚡ con todas las dependencias y configuraciones necesarias 🎯

Perfecto para programar desde cualquier lugar 🌍 sin instalar nada en tu PC 🖥️.

---

**Preparación del entorno de trabajo**  
Una vez hemos  creado la maquina virtual  
 

![](../images/terminal1.png)

ejecutamos lo siguiente:

```plaintext
 bash ./scripts/install.sh
```
---
**Creación de una entidad un “alias” de una billetera:**
Para poder desplegar contratos y hacer operaciones de escritura
en los contratos necesitamos crear una  billetera con fondos en 
la red de pruebas de la siguiente manera:

```plaintext
stellar keys generate --global <entity> --network testnet --fund
stellar keys address <alias>
```
---
**Proyectos ejemplo**

[Hello World🌍](./hello-world/README.md)

[Data types#️⃣](./data-types/README.md)

[if-else✅](./ifelse/README.md)

[Funciones🕶️](./functions/README.md)

[Biblioteca 📜](./library/README.md)

🏘️[**Principal** ](../README.md) 
