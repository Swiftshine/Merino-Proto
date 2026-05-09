#pragma endian big
#pragma array_limit 4294967296
#pragma eval_depth 256
#include <std/array.pat>

import std.sys;
import std.mem;

using f32 = float;


f32 gVersion; // global version

struct Vec2f {
    f32 x;
    f32 y;
};

struct Vec3f {
    f32 x;
    f32 y;
    f32 z;
};
struct String64 {
    char contents[64];
};
struct String32 {
    char contents[32];
};
struct String16 {
    char contents[16];
};

struct VarArray<T> {
    u32 mCount;
    T mObjects[mCount];
};

struct Header {
    u64 mFilesize; // excluding the header
    f32 mVersion; // assumed, based on Epic Yarn documentation
};

enum MapNodeType : u32 {
    MapSet = 0,
    MapPolySet = 1,
    MapObjSet = 2,
    MapItemSet = 3,
    MapEnemySet = 4,
    MapLocator = 5,
    MapPath = 6,
    MapRect = 7,
    MapCircle = 8,
    MapTerrain = 9,
};

struct Params<auto N> {
    s32 mIntParams[N];
    f32 mFloatParams[N];
    String64 mStringParams[N];
};

// under normal circumstances, i'd name unknown variables based on offset
// but because that is never guaranteed to be consistent
// i will instead just mark them by appearance order.
struct MapSetNode {
    MapNodeType mType;

    if (mType == MapNodeType::MapSet) {
        if (gVersion >= 4.70f) {
            s32 m_MS_unk1;
        }
        Vec2f m_MS_BoundsStart;
        Vec2f m_MS_BoundsEnd;
    }
    else if (mType == MapNodeType::MapPolySet) {
        Vec2f m_MPS_Start;
        Vec2f m_MPS_End;
        Vec2f m_MPS_CollisionNormal;
        u32 m_MPS_unk2; // type index of some sort?
        u32 m_MPS_unk3; // type index of some sort?
    }
    else if (mType == MapNodeType::MapObjSet) {
        s32 m_MOS_NameIndex;
        Vec3f m_MOS_Position;
        f32 m_MOS_unk3;
        Vec2f m_MOS_unk4;
        String32 m_MOS_unk5;

        if (gVersion >= 4.43f) {
            s32 m_MOS_unk6;
        }

        if (gVersion >= 4.44f) {
            String32 m_MOS_unk7;
        }

        Vec2f m_MOS_unk8;
        Vec2f m_MOS_unk9;

        if (gVersion >= 4.71f) {
            s32 m_MOS_unk10;
            s32 m_MOS_unk11;
            s32 m_MOS_unk12;
            s32 m_MOS_unk13;
        }

        Params<5> m_MOS_Params;

        if (gVersion >= 4.50f) {
            std::Array<std::Array<String32, 2>, 5> m_MOS_unk14;
        }
    }
    else if (mType == MapNodeType::MapItemSet) {
        s32 m_MIS_NameIndex;
        Vec3f m_MIS_Position;
        f32 m_MIS_unk3;
        Vec2f m_MIS_unk4;
        String32 m_MIS_unk5;

        if (gVersion >= 4.43f) {
            s32 m_MIS_unk6;
        }

        if (gVersion >= 4.44f) {
            String32 m_MIS_unk7;
        }

        Vec2f m_MIS_unk8;
        Vec2f m_MIS_unk9;

        if (gVersion >= 4.71f) {
            s32 m_MIS_unk10;
            s32 m_MIS_unk11;
            s32 m_MIS_unk12;
            s32 m_MIS_unk13;
        }

        Params<5> m_MIS_Params;
    } else if (mType == MapNodeType::MapEnemySet) {
        String32 m_MES_Name;
        String16 m_MES_Direction;
        String16 m_MES_Orientation;
        Vec3f m_MES_Position;

        if (gVersion >= 4.45f) {
            String32 m_MES_unk7;
        }

        if (gVersion < 4.43f) {
            String16 m_MES_unk8;
            String16 m_MES_unk9;
            String32 m_MES_unk10;
            s32 m_MES_unk11;
            s32 m_MES_unk12;
        }

        s32 m_MES_unk13;

        if (gVersion >= 4.42f) {
            s32 m_MES_unk14;
        }

        if (gVersion >= 4.44f) {
            String32 m_MES_unk15;
        }

        f32 m_MES_unk16;
        f32 m_MES_unk17;
        f32 m_MES_unk18;
        f32 m_MES_unk19;

        if (gVersion >= 4.71f) {
            s32 m_MES_unk20;
            s32 m_MES_unk21;
            s32 m_MES_unk22;
            s32 m_MES_unk23;
        }

        if (gVersion >= 4.72f) {
            s32 m_MES_unk24;
        }

        Params<5> m_MES_Params;
    } else if (mType == MapNodeType::MapLocator) {
        String64 m_ML_Name;
        Vec3f m_ML_Position;
        Params<3> m_ML_Params;
    } else if (mType == MapNodeType::MapPath) {
        String64 m_MP_Name;
        VarArray<Vec2f> m_MP_Points;
        Params<3> m_MP_Params;
    } else if (mType == MapNodeType::MapRect) {
        String64 m_MR_Name;
        Vec2f m_MR_BoundsStart;
        Vec2f m_MR_BoundsEnd;
        Params<3> m_MR_Params;
    } else if (mType == MapNodeType::MapCircle) {
        String64 m_MC_Name;
        Vec2f m_MC_Position;
        f32 m_MC_Radius;
        Params<3> m_MC_Params;
    } else if (mType == MapNodeType::MapTerrain) {
        s32 m_MT_NameIndex;
        Vec3f m_MT_Position;

        if (gVersion >= 4.43f) {
            s32 m_MT_unk3;
        }

        if (gVersion >= 4.44f) {
            String32 m_MT_unk4;
        }

        f32 m_MT_unk5;
        f32 m_MT_unk6;
        f32 m_MT_unk7;
        f32 m_MT_unk8;

        if (gVersion >= 4.71f) {
            s32 m_MT_unk9;
            s32 m_MT_unk10;
            s32 m_MT_unk11;
            s32 m_MT_unk12;
        }

        VarArray<std::Array<Vec2f, 3>> m_MT_unk13;

        Params<3> m_MT_Params;

        if (gVersion >= 4.6f) {
            std::Array<std::Array<String32, 2>, 3> m_MT_unk15;
        }
    }


    u32 mChildFlags;

    if ((mChildFlags & 1) != 0) {
        VarArray<MapSetNode> mChildren_MapPolySet;
    }

    if ((mChildFlags & 2) != 0) {
        VarArray<MapSetNode> mChildren_MapObjSet;
    }

    if ((mChildFlags & 4) != 0) {
        VarArray<MapSetNode> mChildren_MapItemSet;
    }

    if ((mChildFlags & 8) != 0) {
        VarArray<MapSetNode> mChildren_MapEnemySet;
    }

    if ((mChildFlags & 0x10) != 0) {
        VarArray<MapSetNode> mChildren_MapLocator;
    }

    if ((mChildFlags & 0x20) != 0) {
        VarArray<MapSetNode> mChildren_MapPath;
    }

    if ((mChildFlags & 0x40) != 0) {
        VarArray<MapSetNode> mChildren_MapRect;
    }

    if ((mChildFlags & 0x80) != 0) {
        VarArray<MapSetNode> mChildren_MapCircle;
    }

    if ((mChildFlags & 0x100) != 0) {
        VarArray<MapSetNode> mChildren_MapTerrain;
    }
};


Header header @ 0;
gVersion = header.mVersion;
VarArray<String32> objectTypes @ $;
VarArray<String32> itemTypes @ $;
VarArray<String32> collisionTypes @ $;
VarArray<String32> rectTypes @ $;
VarArray<String32> enemyTypes @ $;
VarArray<String32> unkTypes1 @ $;
MapSetNode root @ $;
