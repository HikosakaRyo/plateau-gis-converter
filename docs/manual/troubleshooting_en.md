# Troubleshooting

This document describes common issues you may encounter while using PLATEAU GIS Converter and how to resolve them.

## Errors During CityGML to 3D Tiles Conversion

### Error Message: "ConversionFailed: Pipeline thread panicked"

#### Symptoms

When converting CityGML to 3D Tiles, you see an error message like:

```
ConversionFailed: Pipeline thread panicked: Sink thread panicked with message: a scoped thread panicked.
```

#### Causes

This error occurs when a thread within the conversion pipeline encounters an unexpected failure (panic). Common causes include:

1. **Insufficient Memory**
   - System runs out of memory when processing large datasets
   - Data being processed exceeds available RAM

2. **Insufficient Disk Space**
   - Not enough free space on the output destination drive
   - Insufficient space for temporary files

3. **File I/O Errors**
   - No write permission to output directory
   - File system errors
   - Network drive disconnection (when outputting to a network location)

4. **Data Issues**
   - Corrupted CityGML files
   - Invalid geometry data
   - Unsupported CityGML format or version

5. **System Resource Contention**
   - Resource conflicts with other processes
   - Thread pool deadlocks

#### Solutions

##### 1. Check Memory Usage

Check your system's memory usage:

- **Windows**: Check memory usage in Task Manager
- **macOS**: Check memory usage in Activity Monitor

**Solutions**:
- Close other applications to free up available memory
- Split the data into smaller chunks for processing
- Upgrade system RAM

##### 2. Check Disk Space

Verify available disk space at the output location:

- You need at least as much free space as the source data size
- For safety, we recommend having 2x or more free space than source data size

**Solutions**:
- Delete unnecessary files to free up disk space
- Change output destination to a different drive

##### 3. Check File Access Permissions

Ensure you have write permission to the output directory:

**Windows**:
- Right-click output folder → Properties → Security tab to check permissions

**macOS**:
- Select output folder → Get Info (⌘I) → Sharing & Permissions to check permissions

**Solutions**:
- Set output destination within your user directory
- Run application with administrator privileges (not recommended)
- Change output destination to a directory with write permissions

##### 4. Validate Input Data

Verify the integrity of your CityGML files:

- Confirm files are downloaded correctly
- Check if properly structured as XML
- Verify compliance with PLATEAU standard specifications

**Solutions**:
- Re-download the data
- Try with different CityGML files
- Test with smaller sample files

##### 5. Adjust Conversion Settings

Try adjusting these settings:

**For 3D Tiles**:
- Enable texture resolution limiting (`limit_texture_resolution`)
- Start from higher zoom levels (reduce data volume)
- Disable gzip compression (reduce processing load)

**If using command-line version**:
```bash
# Run in debug mode to see detailed logs
RUST_BACKTRACE=1 nusamai input.gml --sink 3dtiles --output output/
```

##### 6. Check Logs

To get detailed error information, obtain logs using these methods:

**GUI Application**:
- Check application log output
- Review logs before and after the error message

**Command-line version**:
```bash
# Enable backtrace with environment variable
RUST_BACKTRACE=full nusamai input.gml --sink 3dtiles --output output/ 2>&1 | tee conversion.log
```

#### Advanced Diagnostic Methods

If the above solutions don't resolve the issue, try these steps to isolate the problem:

1. **Test with Minimal Dataset**
   - Try converting just one small CityGML file
   - If successful, the issue is likely data volume

2. **Test with Other Output Formats**
   - Try converting to GeoJSON or GeoPackage
   - If other formats succeed, the issue is specific to 3D Tiles

3. **Incremental File Addition**
   - Start with 1 file and gradually increase the number
   - Identify the point where the error occurs

4. **Monitor System Resources**
   - Monitor CPU, memory, and disk I/O during conversion
   - Identify which resource is the bottleneck

#### If the Problem Persists

If none of the above solutions work, please report the issue on GitHub with the following information:

- Operating system and version
- PLATEAU GIS Converter version
- Input CityGML file information (file size, region, data type)
- Full error message text
- Runtime logs (obtained with `RUST_BACKTRACE=1`)
- System memory and disk space information

---

## Other Errors

### Conversion Stops Midway

#### Causes
- Network drive connection loss when outputting to network location
- System entering sleep mode
- Processing very large datasets

#### Solutions
- Output to local drive
- Disable sleep in power settings
- Split data for processing

### Cannot Open Output Files

#### Causes
- Incomplete conversion (interrupted midway)
- Using incompatible viewer

#### Solutions
- Complete the conversion to the end
- Use appropriate viewer (e.g., Cesium for 3D Tiles)

---

## Performance Optimization

### How to Improve Conversion Speed

1. **Use Multi-core CPU**
   - Conversion processes are parallelized, so more CPU cores result in faster processing

2. **Use SSD**
   - SSDs have better I/O performance than HDDs, enabling faster processing

3. **Ensure Sufficient Memory**
   - Recommended memory: 3-5x the size of the data being processed

4. **Close Unnecessary Applications**
   - Concentrate system resources on the conversion process
