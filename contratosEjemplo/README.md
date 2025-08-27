**Antes necesitamos un entorno con todo configurado.**

**GitHub Codespaces** es como tener tu propio **editor de código en la nube** ☁️💻.

👉 Abres un **repositorio en GitHub** 📂  
👉 Le das a **Codespaces** ▶️

![](../images/codespaces.png)


  
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
