// src/openapi.rs
//
// OpenAPI 3.0 spec به صورت JSON استاتیک
// بدون هیچ کتابخانه‌ای — مستقیم و بدون خطا

pub fn openapi_spec() -> serde_json::Value {
    serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": "IVR Builder API",
            "version": "1.0.0",
            "description": "سرویس REST برای ساخت و مدیریت فلوهای IVR\n\n## گردش کار\n1. یک **Flow** بسازید\n2. **Node** های مورد نیاز را اضافه کنید\n3. با **Branch** ها گره‌ها را به هم وصل کنید\n4. ورودی اولیه فلو را با `PATCH /api/flows/{id}/entry` تعیین کنید\n\n## انواع گره\n| نوع | کاربرد |\n|-----|--------|\n| `menu` | پخش صوت + دریافت DTMF |\n| `play_audio` | پخش فایل صوتی |\n| `record_audio` | ضبط صدا |\n| `receive_digits` | دریافت چند رقم |\n| `connect_call` | انتقال تماس به اکانت SIP |\n| `hangup` | قطع تماس |"
        },
        "tags": [
            { "name": "Flows",    "description": "مدیریت فلوهای IVR" },
            { "name": "Nodes",    "description": "مدیریت گره‌های فلو" },
            { "name": "Branches", "description": "مدیریت اتصال‌های بین گره‌ها" }
        ],
        "paths": {
            "/api/flows": {
                "get": {
                    "tags": ["Flows"],
                    "summary": "لیست همه فلوها",
                    "operationId": "listFlows",
                    "responses": {
                        "200": { "description": "لیست فلوها", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/IvrFlow" } } } } }
                    }
                },
                "post": {
                    "tags": ["Flows"],
                    "summary": "ایجاد فلو جدید",
                    "operationId": "createFlow",
                    "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/CreateFlowDto" } } } },
                    "responses": {
                        "200": { "description": "فلو ایجاد شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/IvrFlow" } } } },
                        "400": { "description": "ورودی نامعتبر", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                }
            },
            "/api/flows/{flow_id}": {
                "get": {
                    "tags": ["Flows"],
                    "summary": "دریافت فلو کامل",
                    "operationId": "getFlow",
                    "parameters": [{ "name": "flow_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "responses": {
                        "200": { "description": "فلو کامل با گره‌ها و branch ها", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/FlowFull" } } } },
                        "404": { "description": "فلو پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                },
                "put": {
                    "tags": ["Flows"],
                    "summary": "بروزرسانی فلو",
                    "operationId": "updateFlow",
                    "parameters": [{ "name": "flow_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpdateFlowDto" } } } },
                    "responses": {
                        "200": { "description": "فلو بروزرسانی شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/IvrFlow" } } } },
                        "404": { "description": "فلو پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                },
                "delete": {
                    "tags": ["Flows"],
                    "summary": "حذف فلو",
                    "operationId": "deleteFlow",
                    "parameters": [{ "name": "flow_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "responses": {
                        "200": { "description": "فلو حذف شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/DeletedResponse" } } } },
                        "404": { "description": "فلو پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                }
            },
            "/api/flows/{flow_id}/entry": {
                "patch": {
                    "tags": ["Flows"],
                    "summary": "تعیین گره ورودی فلو",
                    "operationId": "setFlowEntry",
                    "parameters": [{ "name": "flow_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/SetEntryDto" } } } },
                    "responses": {
                        "200": { "description": "ورودی تعیین شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/IvrFlow" } } } },
                        "400": { "description": "گره پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                }
            },
            "/api/flows/{flow_id}/nodes": {
                "post": {
                    "tags": ["Nodes"],
                    "summary": "ایجاد گره جدید در فلو",
                    "operationId": "createNode",
                    "parameters": [{ "name": "flow_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/CreateNodeDto" } } } },
                    "responses": {
                        "200": { "description": "گره ایجاد شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/NodeFull" } } } },
                        "400": { "description": "ورودی نامعتبر", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } },
                        "404": { "description": "فلو پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                }
            },
            "/api/nodes/{node_id}": {
                "get": {
                    "tags": ["Nodes"],
                    "summary": "دریافت گره با branch ها",
                    "operationId": "getNode",
                    "parameters": [{ "name": "node_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "responses": {
                        "200": { "description": "اطلاعات گره", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/NodeFull" } } } },
                        "404": { "description": "گره پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                },
                "put": {
                    "tags": ["Nodes"],
                    "summary": "بروزرسانی گره",
                    "operationId": "updateNode",
                    "parameters": [{ "name": "node_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpdateNodeDto" } } } },
                    "responses": {
                        "200": { "description": "گره بروزرسانی شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/NodeFull" } } } },
                        "404": { "description": "گره پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                },
                "delete": {
                    "tags": ["Nodes"],
                    "summary": "حذف گره",
                    "operationId": "deleteNode",
                    "parameters": [{ "name": "node_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "responses": {
                        "200": { "description": "گره حذف شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/DeletedResponse" } } } },
                        "404": { "description": "گره پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                }
            },
            "/api/nodes/{node_id}/branches": {
                "post": {
                    "tags": ["Branches"],
                    "summary": "اضافه کردن branch به گره",
                    "operationId": "createBranch",
                    "parameters": [{ "name": "node_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/CreateBranchDto" } } } },
                    "responses": {
                        "200": { "description": "branch ایجاد شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/IvrBranch" } } } },
                        "400": { "description": "ورودی نامعتبر", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                }
            },
            "/api/branches/{branch_id}": {
                "put": {
                    "tags": ["Branches"],
                    "summary": "بروزرسانی branch",
                    "operationId": "updateBranch",
                    "parameters": [{ "name": "branch_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpdateBranchDto" } } } },
                    "responses": {
                        "200": { "description": "branch بروزرسانی شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/IvrBranch" } } } },
                        "404": { "description": "branch پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                },
                "delete": {
                    "tags": ["Branches"],
                    "summary": "حذف branch",
                    "operationId": "deleteBranch",
                    "parameters": [{ "name": "branch_id", "in": "path", "required": true, "schema": { "type": "string" } }],
                    "responses": {
                        "200": { "description": "branch حذف شد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/DeletedResponse" } } } },
                        "404": { "description": "branch پیدا نشد", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorResponse" } } } }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "IvrFlow": {
                    "type": "object",
                    "properties": {
                        "id":            { "type": "string", "example": "550e8400-e29b-41d4-a716-446655440000" },
                        "name":          { "type": "string", "example": "منوی اصلی" },
                        "description":   { "type": "string", "nullable": true },
                        "entry_node_id": { "type": "string", "nullable": true },
                        "created_at":    { "type": "integer" },
                        "updated_at":    { "type": "integer" }
                    }
                },
                "IvrNode": {
                    "type": "object",
                    "properties": {
                        "id":        { "type": "string" },
                        "flow_id":   { "type": "string" },
                        "node_type": { "type": "string", "enum": ["menu","play_audio","record_audio","receive_digits","connect_call","hangup"] },
                        "label":     { "type": "string", "nullable": true },
                        "config":    { "type": "object" },
                        "created_at":{ "type": "integer" }
                    }
                },
                "IvrBranch": {
                    "type": "object",
                    "properties": {
                        "id":           { "type": "string" },
                        "node_id":      { "type": "string" },
                        "digit":        { "type": "string", "enum": ["0","1","2","3","4","5","6","7","8","9","*","#","timeout","invalid"] },
                        "next_node_id": { "type": "string", "nullable": true },
                        "label":        { "type": "string", "nullable": true },
                        "created_at":   { "type": "integer" }
                    }
                },
                "NodeFull": {
                    "type": "object",
                    "allOf": [{ "$ref": "#/components/schemas/IvrNode" }],
                    "properties": {
                        "branches": { "type": "array", "items": { "$ref": "#/components/schemas/IvrBranch" } }
                    }
                },
                "FlowFull": {
                    "type": "object",
                    "allOf": [{ "$ref": "#/components/schemas/IvrFlow" }],
                    "properties": {
                        "nodes": { "type": "array", "items": { "$ref": "#/components/schemas/NodeFull" } }
                    }
                },
                "CreateFlowDto": {
                    "type": "object",
                    "required": ["name"],
                    "properties": {
                        "name":        { "type": "string", "example": "منوی اصلی" },
                        "description": { "type": "string", "nullable": true }
                    }
                },
                "UpdateFlowDto": {
                    "type": "object",
                    "properties": {
                        "name":        { "type": "string" },
                        "description": { "type": "string", "nullable": true }
                    }
                },
                "SetEntryDto": {
                    "type": "object",
                    "required": ["node_id"],
                    "properties": {
                        "node_id": { "type": "string" }
                    }
                },
                "CreateNodeDto": {
                    "type": "object",
                    "required": ["node_type", "config"],
                    "properties": {
                        "node_type": { "type": "string", "enum": ["menu","play_audio","record_audio","receive_digits","connect_call","hangup"] },
                        "label":     { "type": "string", "nullable": true },
                        "config":    { "type": "object", "example": { "audio_file": "welcome.wav", "timeout_secs": 5 } }
                    }
                },
                "UpdateNodeDto": {
                    "type": "object",
                    "properties": {
                        "label":  { "type": "string", "nullable": true },
                        "config": { "type": "object" }
                    }
                },
                "CreateBranchDto": {
                    "type": "object",
                    "required": ["digit"],
                    "properties": {
                        "digit":        { "type": "string", "enum": ["0","1","2","3","4","5","6","7","8","9","*","#","timeout","invalid"] },
                        "next_node_id": { "type": "string", "nullable": true },
                        "label":        { "type": "string", "nullable": true }
                    }
                },
                "UpdateBranchDto": {
                    "type": "object",
                    "properties": {
                        "digit":        { "type": "string" },
                        "next_node_id": { "type": "string", "nullable": true },
                        "label":        { "type": "string", "nullable": true }
                    }
                },
                "DeletedResponse": {
                    "type": "object",
                    "properties": {
                        "deleted": { "type": "string" }
                    }
                },
                "ErrorResponse": {
                    "type": "object",
                    "properties": {
                        "error": { "type": "string" }
                    }
                }
            }
        }
    })
}